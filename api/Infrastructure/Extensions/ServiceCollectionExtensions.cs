using Amazon.S3;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;
using Microsoft.IdentityModel.Tokens;
using Nest;
using ShortasProxyApi.Application.Services;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;
using ShortasProxyApi.Infrastructure.HttpClients;
using ShortasProxyApi.Infrastructure.Security;
using ShortasProxyApi.Infrastructure.Services;
using ShortasProxyApi.Infrastructure.Repositories;
using ShortasProxyApi.Infrastructure.BackgroundServices;
using Polly;
using Polly.Extensions.Http;
using System.Text;

namespace ShortasProxyApi.Infrastructure;

public static class ServiceCollectionExtensions
{
    public static IServiceCollection AddInfrastructureServices(this IServiceCollection services, IConfiguration configuration)
    {
        // Add Entity Framework
        services.AddDbContext<ApplicationDbContext>(options =>
            options.UseNpgsql(configuration.GetConnectionString("DefaultConnection")));

        // Add Elasticsearch
        var elasticsearchUrl = configuration["Elasticsearch:Url"] ?? "http://localhost:9200";
        var connectionSettings = new ConnectionSettings(new Uri(elasticsearchUrl))
            .DefaultMappingFor<RouteSearchDocument>(m => m
                .IndexName(configuration["Elasticsearch:IndexName"] ?? "routes")
                .IdProperty(p => p.Id)
            )
            .EnableDebugMode()
            .ThrowExceptions(false);

        services.AddSingleton<IElasticClient>(new ElasticClient(connectionSettings));
        services.AddScoped<IRouteSearchService, ElasticsearchRouteSearchService>();

        // Add HTTP clients with Polly for resilience
        services.AddHttpClient("ClickRouterApi", client =>
        {
            var baseUrl = configuration["ApiSettings:ClickRouterApi:BaseUrl"] ?? "http://localhost:8081";
            var timeout = configuration.GetValue<int>("ApiSettings:ClickRouterApi:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        // Register the HTTP client class manually
        services.AddScoped<ClickRouterApiClient>(provider =>
        {
            var httpClientFactory = provider.GetRequiredService<IHttpClientFactory>();
            var httpClient = httpClientFactory.CreateClient("ClickRouterApi");
            var logger = provider.GetRequiredService<ILogger<ClickRouterApiClient>>();
            return new ClickRouterApiClient(httpClient, logger);
        });

        services.AddHttpClient<ClickAggregatorApiClient>(client =>
        {
            var baseUrl = configuration["ApiSettings:ClickAggregatorApi:BaseUrl"] ?? "http://localhost:8082";
            var timeout = configuration.GetValue<int>("ApiSettings:ClickAggregatorApi:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        // Register Repositories
        services.AddScoped<IOutboxRepository, OutboxRepository>();

        // Register EF services for database access with eventual consistency
        services.AddScoped<ISlashTagGenerator, SlashTagGenerator>();
        services.AddScoped<IRouteService, EfRouteService>();
        services.AddScoped<IDomainService, EfDomainService>();
        services.AddScoped<IWorkspaceService, EfWorkspaceService>();
        services.AddScoped<EfCertificateService>();
        services.AddScoped<EfUserSettingsService>();

        // Register HTTP client services for direct API communication (used by outbox processor)
        services.AddScoped<ClickRouterApiService>();
        services.AddScoped<ICertificateService, ClickRouterApiService>();
        services.AddScoped<IUserSettingsService, ClickRouterApiService>();
        services.AddScoped<IClickStreamService, ClickAggregatorApiService>();

        // Register HTTP client for Outbox background service
        services.AddHttpClient("OutboxClickRouterApi", client =>
        {
            var baseUrl = configuration["ApiSettings:ClickRouterApi:BaseUrl"] ?? "http://localhost:8081";
            var timeout = configuration.GetValue<int>("ApiSettings:ClickRouterApi:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        // Register HTTP client for Domain Verifier
        services.AddHttpClient("DomainVerifier", client =>
        {
            var baseUrl = configuration["ApiSettings:DomainVerifier:BaseUrl"] ?? "http://localhost:5830";
            var timeout = configuration.GetValue<int>("ApiSettings:DomainVerifier:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        // Register HTTP client for Route Verifier (Safe Browsing)
        services.AddHttpClient("RouteVerifier", client =>
        {
            var baseUrl = configuration["ApiSettings:RouteVerifier:BaseUrl"] ?? "http://localhost:5831";
            var timeout = configuration.GetValue<int>("ApiSettings:RouteVerifier:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        // Register Outbox background service
        services.AddHostedService<OutboxProcessorService>();

        // Register Domain Verification Consumer
        services.AddHostedService<DomainVerificationConsumer>();

        // Register Route Status Consumer (for Safe Browsing verification events)
        services.AddHostedService<RouteStatusConsumer>();

        // Add S3/MinIO Object Storage
        var s3Endpoint = configuration["S3:Endpoint"] ?? "http://localhost:9000";
        var s3AccessKey = configuration["S3:AccessKey"] ?? "minioadmin";
        var s3SecretKey = configuration["S3:SecretKey"] ?? "minioadmin";
        var usePathStyle = configuration.GetValue<bool>("S3:UsePathStyle", true);

        var s3Config = new AmazonS3Config
        {
            ServiceURL = s3Endpoint,
            ForcePathStyle = usePathStyle,
            UseHttp = s3Endpoint.StartsWith("http://")
        };

        services.AddSingleton<IAmazonS3>(new AmazonS3Client(s3AccessKey, s3SecretKey, s3Config));
        services.AddScoped<IObjectStorageService, MinioObjectStorageService>();

        // Add JWT Authentication
        services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
            .AddJwtBearer(options =>
            {
                options.Authority = configuration["Keycloak:Authority"];
                options.Audience = configuration["Keycloak:Audience"];
                options.RequireHttpsMetadata = configuration.GetValue<bool>("Keycloak:RequireHttpsMetadata", false);
                options.TokenValidationParameters = new TokenValidationParameters
                {
                    ValidateIssuer = true,
                    ValidateAudience = true,
                    ValidateLifetime = true,
                    ValidateIssuerSigningKey = true,
                    ClockSkew = TimeSpan.Zero
                };
            });

        services.AddAuthorization();

        return services;
    }

    private static IAsyncPolicy<HttpResponseMessage> GetRetryPolicy()
    {
        return HttpPolicyExtensions
            .HandleTransientHttpError()
            // Don't retry on 404 Not Found - it's not a transient error and our app handles it
            .OrResult(msg => !msg.IsSuccessStatusCode && msg.StatusCode != System.Net.HttpStatusCode.NotFound)
            .WaitAndRetryAsync(
                retryCount: 3,
                sleepDurationProvider: retryAttempt => TimeSpan.FromSeconds(Math.Pow(2, retryAttempt)) + TimeSpan.FromMilliseconds(new Random().Next(0, 1000)),
                onRetry: (outcome, timespan, retryCount, context) =>
                {
                    Console.WriteLine($"Retry {retryCount} in {timespan} seconds due to: {outcome.Result?.StatusCode}");
                });
    }

    private static IAsyncPolicy<HttpResponseMessage> GetCircuitBreakerPolicy()
    {
        return HttpPolicyExtensions
            .HandleTransientHttpError()
            // Don't break circuit on 404 Not Found - it doesn't indicate service degradation
            .OrResult(msg => !msg.IsSuccessStatusCode && msg.StatusCode != System.Net.HttpStatusCode.NotFound)
            .CircuitBreakerAsync(
                handledEventsAllowedBeforeBreaking: 3,
                durationOfBreak: TimeSpan.FromSeconds(30),
                onBreak: (exception, duration) =>
                {
                    Console.WriteLine($"Circuit breaker opened for {duration} due to: {exception}");
                },
                onReset: () =>
                {
                    Console.WriteLine("Circuit breaker reset");
                },
                onHalfOpen: () =>
                {
                    Console.WriteLine("Circuit breaker half-open");
                });
    }

    private static IAsyncPolicy<HttpResponseMessage> GetTimeoutPolicy()
    {
        return Polly.Policy.TimeoutAsync<HttpResponseMessage>(TimeSpan.FromSeconds(10));
    }
}