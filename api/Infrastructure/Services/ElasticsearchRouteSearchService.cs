using Nest;
using ShortasProxyApi.Domain.Interfaces;
using Result = ShortasProxyApi.Domain.Common.Result;
using ResultT = ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Infrastructure.Services;

/// <summary>
/// Elasticsearch-backed implementation of IRouteSearchService.
/// Indexes routes for full-text search across link, domain name, and destination URL.
/// </summary>
public class ElasticsearchRouteSearchService : IRouteSearchService
{
    private readonly IElasticClient _client;
    private readonly ILogger<ElasticsearchRouteSearchService> _logger;
    private readonly string _indexName;

    public ElasticsearchRouteSearchService(
        IElasticClient client,
        IConfiguration configuration,
        ILogger<ElasticsearchRouteSearchService> logger)
    {
        _client = client;
        _logger = logger;
        _indexName = configuration["Elasticsearch:IndexName"] ?? "routes";
    }

    public async Task EnsureIndexAsync()
    {
        var indexExists = await _client.Indices.ExistsAsync(_indexName);
        if (indexExists.Exists)
            return;

        _logger.LogInformation("Creating Elasticsearch index: {IndexName}", _indexName);

        var createResponse = await _client.Indices.CreateAsync(_indexName, c => c
            .Settings(s => s
                .NumberOfShards(1)
                .NumberOfReplicas(0)
                .Analysis(a => a
                    .Analyzers(an => an
                        .Custom("route_analyzer", ca => ca
                            .Tokenizer("standard")
                            .Filters("lowercase", "asciifolding")
                        )
                    )
                )
            )
            .Map<RouteSearchDocument>(m => m
                .Properties(p => p
                    .Keyword(k => k.Name(n => n.Id))
                    .Text(t => t
                        .Name(n => n.Link)
                        .Analyzer("route_analyzer")
                        .Fields(f => f.Keyword(kw => kw.Name("keyword")))
                    )
                    .Text(t => t
                        .Name(n => n.Switch)
                        .Analyzer("route_analyzer")
                        .Fields(f => f.Keyword(kw => kw.Name("keyword")))
                    )
                    .Text(t => t
                        .Name(n => n.Dest)
                        .Analyzer("route_analyzer")
                    )
                    .Text(t => t
                        .Name(n => n.DomainName)
                        .Analyzer("route_analyzer")
                        .Fields(f => f.Keyword(kw => kw.Name("keyword")))
                    )
                    .Keyword(k => k.Name(n => n.Status))
                    .Keyword(k => k.Name(n => n.OwnerId))
                    .Keyword(k => k.Name(n => n.WorkspaceId))
                )
            )
        );

        if (!createResponse.IsValid)
        {
            _logger.LogError("Failed to create Elasticsearch index: {Error}", createResponse.DebugInformation);
        }
    }

    public async Task<Result> IndexRouteAsync(RouteSearchDocument document)
    {
        try
        {
            var response = await _client.IndexAsync(document, i => i
                .Index(_indexName)
                .Id(document.Id)
                .Refresh(Elasticsearch.Net.Refresh.WaitFor)
            );

            if (!response.IsValid)
            {
                _logger.LogError("Failed to index route {RouteId}: {Error}", document.Id, response.DebugInformation);
                return Result.Failure("SEARCH_INDEX_ERROR", $"Failed to index route: {response.ServerError?.Error?.Reason}");
            }

            _logger.LogDebug("Indexed route {RouteId} in search", document.Id);
            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error indexing route {RouteId}", document.Id);
            return Result.Failure("SEARCH_INDEX_ERROR", "Failed to index route in search");
        }
    }

    public async Task<Result> IndexRoutesAsync(List<RouteSearchDocument> documents)
    {
        if (documents.Count == 0)
            return Result.Success();

        try
        {
            var response = await _client.BulkAsync(b => b
                .Index(_indexName)
                .IndexMany(documents, (descriptor, doc) => descriptor.Id(doc.Id))
                .Refresh(Elasticsearch.Net.Refresh.WaitFor)
            );

            if (!response.IsValid)
            {
                _logger.LogError("Failed to bulk index {Count} routes: {Error}", documents.Count, response.DebugInformation);
                return Result.Failure("SEARCH_INDEX_ERROR", "Failed to bulk index routes");
            }

            if (response.Errors)
            {
                var failedCount = response.ItemsWithErrors.Count();
                _logger.LogWarning("Bulk index completed with {FailedCount}/{TotalCount} failures", failedCount, documents.Count);
            }

            _logger.LogDebug("Bulk indexed {Count} routes in search", documents.Count);
            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error bulk indexing {Count} routes", documents.Count);
            return Result.Failure("SEARCH_INDEX_ERROR", "Failed to bulk index routes in search");
        }
    }

    public async Task<Result> DeleteRouteAsync(string routeId)
    {
        try
        {
            var response = await _client.DeleteAsync<RouteSearchDocument>(routeId, d => d
                .Index(_indexName)
                .Refresh(Elasticsearch.Net.Refresh.WaitFor)
            );

            if (!response.IsValid && response.Result != Nest.Result.NotFound)
            {
                _logger.LogError("Failed to delete route {RouteId} from search: {Error}", routeId, response.DebugInformation);
                return Result.Failure("SEARCH_INDEX_ERROR", "Failed to delete route from search");
            }

            _logger.LogDebug("Deleted route {RouteId} from search", routeId);
            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error deleting route {RouteId} from search", routeId);
            return Result.Failure("SEARCH_INDEX_ERROR", "Failed to delete route from search");
        }
    }

    public async Task<Result> DeleteRoutesAsync(List<string> routeIds)
    {
        if (routeIds.Count == 0)
            return Result.Success();

        try
        {
            var response = await _client.BulkAsync(b => b
                .Index(_indexName)
                .DeleteMany<RouteSearchDocument>(
                    routeIds.Select(id => new RouteSearchDocument { Id = id }),
                    (descriptor, doc) => descriptor.Id(doc.Id))
                .Refresh(Elasticsearch.Net.Refresh.WaitFor)
            );

            if (!response.IsValid)
            {
                _logger.LogError("Failed to bulk delete {Count} routes from search: {Error}", routeIds.Count, response.DebugInformation);
                return Result.Failure("SEARCH_INDEX_ERROR", "Failed to bulk delete routes from search");
            }

            _logger.LogDebug("Bulk deleted {Count} routes from search", routeIds.Count);
            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error bulk deleting {Count} routes from search", routeIds.Count);
            return Result.Failure("SEARCH_INDEX_ERROR", "Failed to bulk delete routes from search");
        }
    }

    public async Task<ResultT.Result<(List<RouteSearchDocument> Results, long TotalCount)>> SearchAsync(
        string query,
        string? ownerId = null,
        string? workspaceId = null,
        int page = 1,
        int pageSize = 20)
    {
        try
        {
            var response = await _client.SearchAsync<RouteSearchDocument>(s => s
                .Index(_indexName)
                .From((page - 1) * pageSize)
                .Size(pageSize)
                .Query(q => q
                    .Bool(b =>
                    {
                        var must = new List<Func<QueryContainerDescriptor<RouteSearchDocument>, QueryContainer>>();

                        // Full-text search across link, domain name, destination, and switch
                        must.Add(m => m.MultiMatch(mm => mm
                            .Query(query)
                            .Fields(f => f
                                .Field(ff => ff.Link, boost: 2.0)
                                .Field(ff => ff.DomainName, boost: 2.0)
                                .Field(ff => ff.Dest)
                                .Field(ff => ff.Switch)
                            )
                            .Type(TextQueryType.BestFields)
                            .Fuzziness(Fuzziness.Auto)
                        ));

                        // Filter by owner (mandatory for multi-tenancy)
                        if (!string.IsNullOrEmpty(ownerId))
                            must.Add(m => m.Term(t => t.Field(f => f.OwnerId).Value(ownerId)));

                        // Filter by workspace
                        if (!string.IsNullOrEmpty(workspaceId))
                            must.Add(m => m.Term(t => t.Field(f => f.WorkspaceId).Value(workspaceId)));

                        return b.Must(must.ToArray());
                    })
                )
                .Highlight(h => h
                    .Fields(
                        f => f.Field(ff => ff.Link),
                        f => f.Field(ff => ff.DomainName),
                        f => f.Field(ff => ff.Dest)
                    )
                    .PreTags("<em>")
                    .PostTags("</em>")
                )
            );

            if (!response.IsValid)
            {
                _logger.LogError("Search query failed: {Error}", response.DebugInformation);
                return ResultT.Result<(List<RouteSearchDocument>, long)>.Failure("SEARCH_ERROR", "Search query failed");
            }

            var results = response.Documents.ToList();
            return ResultT.Result<(List<RouteSearchDocument> Results, long TotalCount)>.Success((results, response.Total));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error executing search query: {Query}", query);
            return ResultT.Result<(List<RouteSearchDocument>, long)>.Failure("SEARCH_ERROR", "Failed to execute search query");
        }
    }
}
