//! Elasticsearch service for route full-text search.

use elasticsearch::{
    http::transport::Transport, DeleteParts, Elasticsearch, IndexParts, SearchParts,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::domain::entities::Route;
use crate::settings::ElasticsearchSettings;

/// Elasticsearch search service.
pub struct ElasticsearchService {
    client: Elasticsearch,
    routes_index: String,
}

/// Route document for Elasticsearch indexing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDocument {
    pub id: String,
    pub link: String,
    pub dest: Option<String>,
    pub domain_id: Option<String>,
    pub owner_id: Option<String>,
    pub workspace_id: Option<String>,
    pub tags: Vec<String>,
    pub status: String,
    pub created_at: Option<String>,
}

impl From<&Route> for RouteDocument {
    fn from(route: &Route) -> Self {
        Self {
            id: route.id.to_string(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            domain_id: route.domain_id.map(|d| d.to_string()),
            owner_id: route.properties.owner_id.clone(),
            workspace_id: route.properties.workspace_id.clone(),
            tags: route.properties.tags.clone().unwrap_or_default(),
            status: route.status.as_str().to_string(),
            created_at: None,
        }
    }
}

/// Search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub route_ids: Vec<Uuid>,
    pub total_count: i64,
}

impl ElasticsearchService {
    /// Create a new Elasticsearch service.
    pub fn new(settings: &ElasticsearchSettings) -> anyhow::Result<Self> {
        let transport = Transport::single_node(&settings.url)?;
        let client = Elasticsearch::new(transport);

        Ok(Self {
            client,
            routes_index: settings.routes_index.clone(),
        })
    }

    /// Index a route.
    pub async fn index_route(&self, route: &Route) -> anyhow::Result<()> {
        let doc = RouteDocument::from(route);

        self.client
            .index(IndexParts::IndexId(&self.routes_index, &doc.id))
            .body(&doc)
            .send()
            .await?;

        Ok(())
    }

    /// Delete a route from the index.
    pub async fn delete_route(&self, route_id: Uuid) -> anyhow::Result<()> {
        let _ = self
            .client
            .delete(DeleteParts::IndexId(&self.routes_index, &route_id.to_string()))
            .send()
            .await;

        Ok(())
    }

    /// Search routes by query.
    pub async fn search(
        &self,
        query: &str,
        owner_id: Option<&str>,
        workspace_id: Option<&str>,
        page: i32,
        page_size: i32,
    ) -> anyhow::Result<SearchResult> {
        let from = (page - 1) * page_size;

        // Build query with filters
        let mut must = vec![json!({
            "multi_match": {
                "query": query,
                "fields": ["link^3", "dest^2", "tags"],
                "fuzziness": "AUTO"
            }
        })];

        if let Some(oid) = owner_id {
            must.push(json!({ "term": { "owner_id": oid } }));
        }

        if let Some(wid) = workspace_id {
            must.push(json!({ "term": { "workspace_id": wid } }));
        }

        let search_body = json!({
            "query": {
                "bool": {
                    "must": must
                }
            },
            "from": from,
            "size": page_size,
            "sort": [
                { "_score": "desc" },
                { "created_at": "desc" }
            ]
        });

        let response = self
            .client
            .search(SearchParts::Index(&[&self.routes_index]))
            .body(search_body)
            .send()
            .await?;

        let response_body = response.json::<serde_json::Value>().await?;

        let total_count = response_body["hits"]["total"]["value"]
            .as_i64()
            .unwrap_or(0);

        let route_ids: Vec<Uuid> = response_body["hits"]["hits"]
            .as_array()
            .map(|hits| {
                hits.iter()
                    .filter_map(|hit| {
                        hit["_id"]
                            .as_str()
                            .and_then(|id| Uuid::parse_str(id).ok())
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(SearchResult {
            route_ids,
            total_count,
        })
    }

    /// Bulk index routes.
    pub async fn bulk_index(&self, routes: &[Route]) -> anyhow::Result<()> {
        if routes.is_empty() {
            return Ok(());
        }

        let mut body: Vec<serde_json::Value> = Vec::with_capacity(routes.len() * 2);

        for route in routes {
            let doc = RouteDocument::from(route);
            body.push(json!({ "index": { "_index": &self.routes_index, "_id": &doc.id } }));
            body.push(serde_json::to_value(&doc)?);
        }

        let body_str: String = body
            .iter()
            .map(|v| serde_json::to_string(v).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";

        self.client
            .bulk(elasticsearch::BulkParts::None)
            .body(vec![body_str])
            .send()
            .await?;

        Ok(())
    }

    /// Check if Elasticsearch is available.
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        let response = self.client.ping().send().await?;
        Ok(response.status_code().is_success())
    }
}
