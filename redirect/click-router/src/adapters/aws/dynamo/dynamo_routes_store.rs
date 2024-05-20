use std::collections::HashMap;
use std::u128;

use anyhow::{Error, Result};

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use http::StatusCode;
use serde_dynamo::aws_sdk_dynamodb_1::from_attribute_value;
use serde_json::Value;

use crate::core::BaseRoutesStore;
use crate::model::condition::Condition;
use crate::model::route::{
    BlockedReason, ChallengeRouting, ConditionalRouting, DestinationFormat, FileRouting,
    RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal,
};
use crate::model::Route;

#[derive(Clone, Debug)]
pub struct DynamoRoutesStore {
    client: Client,
    routes_table: String,
}

impl DynamoRoutesStore {
    pub fn new(sdk_config: &SdkConfig, routes_table: String) -> Self {
        Self {
            routes_table,
            client: Client::new(sdk_config),
        }
    }

    fn to_terminal(
        &self,
        routing_item: &HashMap<String, AttributeValue>,
    ) -> Result<RoutingTerminal> {
        if routing_item.get("terminal").is_none() {
            Err(Error::msg("Could not find 'routing.terminal' attribute."))?;
        }

        let routing_terminal =
            routing_item
                .get("terminal")
                .map_or(RoutingTerminal::External, |policy| {
                    let terminal_type = policy.as_s().unwrap().to_ascii_lowercase();

                    if terminal_type == "internal" {
                        return RoutingTerminal::Internal;
                    } else if terminal_type == "middleware" {
                        return RoutingTerminal::Middleware;
                    } else {
                        return RoutingTerminal::External;
                    }
                });

        Ok(routing_terminal)
    }

    fn to_conditional_policy(
        &self,
        routing_item: &HashMap<String, AttributeValue>,
    ) -> Result<RoutingPolicy> {
        if routing_item.get("conditions").is_none() {
            Err(Error::msg("Could not find 'routing.conditions' attribute."))?;
        }

        let conditions = routing_item.get("conditions").map_or(Ok(Vec::new()), |c| {
            let conditions_items = c.as_l().unwrap();

            let result: Result<Vec<ConditionalRouting>> = conditions_items
                .iter()
                .map(|i| {
                    let condition_item = i.as_m().unwrap();

                    if condition_item.get("key").is_none() {
                        Err(Error::msg(
                            "Could not find 'routing.conditions.key' attribute.",
                        ))?;
                    }

                    if condition_item.get("condition").is_none() {
                        Err(Error::msg(
                            "Could not find 'routing.conditions.condition' attribute.",
                        ))?;
                    }

                    let key = condition_item
                        .get("key")
                        .unwrap()
                        .as_s()
                        .unwrap()
                        .to_ascii_lowercase();

                    let condition_json = condition_item.get("condition").unwrap().as_s().unwrap();

                    let condition: Condition = serde_json::from_str(condition_json).unwrap();

                    return Ok(ConditionalRouting { key, condition });
                })
                .collect();

            result
        });

        return Ok(RoutingPolicy::Conditional(conditions?));
    }

    fn to_challenge_policy(
        &self,
        routing_item: &HashMap<String, AttributeValue>,
    ) -> Result<RoutingPolicy> {
        if routing_item.get("challenge").is_none() {
            Err(Error::msg("Could not find 'routing.challenge' attribute."))?;
        }

        let challenge_item = routing_item.get("challenge").unwrap().as_m().unwrap();

        if challenge_item.get("type").is_none() {
            Err(Error::msg(
                "Could not find 'routing.challenge.type' attribute.",
            ))?;
        }

        if challenge_item.get("key").is_none() {
            Err(Error::msg(
                "Could not find 'routing.challenge.key' attribute.",
            ))?;
        }

        if challenge_item.get("source").is_none() {
            Err(Error::msg(
                "Could not find 'routing.challenge.source' attribute.",
            ))?;
        }

        let challenge_type = challenge_item.get("type").unwrap().as_s().unwrap();
        let key = challenge_item.get("key").unwrap().as_s().unwrap();
        let source = challenge_item.get("source").unwrap().as_s().unwrap();

        let routing = ChallengeRouting {
            challenge_type: challenge_type.to_ascii_lowercase(),
            key: key.to_ascii_lowercase(),
            source: source.to_ascii_lowercase(),
        };

        return Ok(RoutingPolicy::Challenge(routing));
    }

    fn to_file_policy(
        &self,
        routing_item: &HashMap<String, AttributeValue>,
    ) -> Result<RoutingPolicy> {
        if routing_item.get("file").is_none() {
            Err(Error::msg("Could not find 'routing.file' attribute."))?;
        }

        let file_item = routing_item.get("file").unwrap().as_m().unwrap();

        if file_item.get("content_type").is_none() {
            Err(Error::msg(
                "Could not find 'routing.file.content_type' attribute.",
            ))?;
        }

        let content_type = file_item.get("content_type").unwrap().as_s().unwrap();

        let routing = FileRouting {
            content_type: content_type.to_string(),
        };

        return Ok(RoutingPolicy::File(routing));
    }

    fn to_policy(&self, routing_item: &HashMap<String, AttributeValue>) -> Result<RoutingPolicy> {
        if routing_item.get("policy").is_none() {
            Err(Error::msg("Could not find 'policy' attribute."))?;
        }

        let routing_policy =
            routing_item
                .get("policy")
                .map_or(Ok(RoutingPolicy::Basic), |policy| {
                    let policy_type = policy.as_s().unwrap().to_ascii_lowercase();

                    if policy_type == "conditional" {
                        return self.to_conditional_policy(routing_item);
                    } else if policy_type == "challenge" {
                        return self.to_challenge_policy(routing_item);
                    } else if policy_type == "file" {
                        return self.to_file_policy(routing_item);
                    } else if policy_type == "mirroring" {
                        return Ok(RoutingPolicy::Mirroring);
                    } else {
                        return Ok(RoutingPolicy::Basic);
                    }
                });

        routing_policy
    }

    fn to_entity(&self, model: GetItemOutput) -> Result<Option<Route>> {
        model.item.map_or(Ok(None), |item| {
            let switch_str = String::from(item.get("switch").unwrap().as_s().unwrap());
            let link_str = String::from(item.get("link").unwrap().as_s().unwrap());
            let dest_format = item
                .get("dest.format")
                .map_or(DestinationFormat::Http, |d| {
                    let dest_format = d.as_s().unwrap().to_ascii_lowercase();

                    if dest_format == "native" {
                        return DestinationFormat::Native;
                    } else {
                        return DestinationFormat::Http;
                    }
                });

            let dest = item.get("dest").map_or(None, |d| {
                let dest = d.as_s().unwrap();
                let dest = urlencoding::decode(dest).unwrap().to_string();
                Some(dest)
            });

            let status_code = item
                .get("code")
                .map_or(StatusCode::TEMPORARY_REDIRECT, |d| {
                    let code = d.as_n().unwrap().parse().unwrap();

                    StatusCode::from_u16(code).unwrap()
                });

            let status = item.get("blocked").map_or(RouteStatus::Active, |d| {
                let blocked = d.as_bool().unwrap();

                if *blocked {
                    let blocked_reason =
                        item.get("blocked.reason")
                            .map_or(BlockedReason::Unknown, |r| {
                                let blocked_reason = r.as_s().unwrap().to_string();
                                BlockedReason::Resoned(blocked_reason)
                            });

                    return RouteStatus::Blocked(blocked_reason);
                } else {
                    return RouteStatus::Active;
                }
            });

            let ttl = item
                .get("ttl")
                .map_or(None, |d| Some(d.as_n().unwrap().parse::<u128>().unwrap()));

            //properties
            let domain_id = item
                .get("domain.id")
                .map_or(None, |d| Some(String::from(d.as_s().unwrap())));

            let route_id = item
                .get("route.id")
                .map_or(None, |d| Some(String::from(d.as_s().unwrap())));

            let owner_id = item
                .get("owner.id")
                .map_or(None, |d| Some(String::from(d.as_s().unwrap())));

            let creator_id = item
                .get("creator.id")
                .map_or(None, |d| Some(String::from(d.as_s().unwrap())));

            let workspace_id = item
                .get("workspace.id")
                .map_or(None, |d| Some(String::from(d.as_s().unwrap())));

            let scripts = item
                .get("script.ids")
                .map_or(None, |d| Some(d.as_ss().unwrap().clone()));

            let tags = item
                .get("script.ids")
                .map_or(None, |d| Some(d.as_ss().unwrap().clone()));

            //let custom_json = serde_json::to_string(&custom).unwrap();
            let custom: Option<Value> = item
                .get("attributes")
                .map_or(None, |p| Some(from_attribute_value(p.to_owned()).unwrap()));

            let native: Option<Value> = item
                .get("native")
                .map_or(None, |p| Some(from_attribute_value(p.to_owned()).unwrap()));

            let bundling: Option<Value> = item
                .get("bundling")
                .map_or(None, |p| Some(from_attribute_value(p.to_owned()).unwrap()));

            let opengraph = item.get("blocked").map_or(false, |d| {
                *d.as_bool().unwrap()
            });
            let properties = RouteProperties {
                creator_id: creator_id,
                owner_id: owner_id,
                domain_id: domain_id,
                route_id: route_id,
                workspace_id: workspace_id,
                scripts: scripts,
                tags: tags,
                custom: custom,
                native: native,
                bundling: bundling,
                opengraph: opengraph,
            };

            //policy
            let routing_policy = item.get("routing").map_or(Ok(RoutingPolicy::Unknown), |d| {
                if let Ok(routing_item) = d.as_m() {
                    return self.to_policy(routing_item);
                }

                Ok(RoutingPolicy::Basic)
            });

            //terminal
            let terminal = item
                .get("routing")
                .map_or(Ok(RoutingTerminal::External), |d| {
                    if let Ok(routing_item) = d.as_m() {
                        return self.to_terminal(routing_item);
                    }

                    Ok(RoutingTerminal::External)
                });

            let route = Route::new(
                switch_str,
                link_str,
                dest,
                dest_format,
                status_code,
                status,
                ttl,
                terminal?,
                routing_policy?,
                properties,
            );

            Ok(Some(route))
        })
    }
}

#[async_trait::async_trait(?Send)]
impl BaseRoutesStore for DynamoRoutesStore {
    async fn invalidate(&self, _: &str, _: &str) -> Result<()> {
        Ok(())
    }

    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.routes_table)
            .set_key(Some(HashMap::from([
                (
                    "link".to_string(),
                    AttributeValue::S(path.to_ascii_lowercase()),
                ),
                (
                    "switch".to_string(),
                    AttributeValue::S(switch.to_ascii_lowercase()),
                ),
            ])))
            .send()
            .await?;

        Ok(self.to_entity(item)?)
    }
}
