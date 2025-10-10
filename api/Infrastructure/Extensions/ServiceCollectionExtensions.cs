using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;
using Microsoft.IdentityModel.Tokens;
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

        // Add HTTP clients with Polly for resilience
        services.AddHttpClient<RouteService>("ClickRouterApi", client =>
        {
            var baseUrl = configuration["ApiSettings:ClickRouterApi:BaseUrl"] ?? "http://localhost:8081";
            var timeout = configuration.GetValue<int>("ApiSettings:ClickRouterApi:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        services.AddHttpClient<CertificateService>("ClickRouterApi", client =>
        {
            var baseUrl = configuration["ApiSettings:ClickRouterApi:BaseUrl"] ?? "http://localhost:8081";
            var timeout = configuration.GetValue<int>("ApiSettings:ClickRouterApi:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        services.AddHttpClient<UserSettingsService>("ClickRouterApi", client =>
        {
            var baseUrl = configuration["ApiSettings:ClickRouterApi:BaseUrl"] ?? "http://localhost:8081";
            var timeout = configuration.GetValue<int>("ApiSettings:ClickRouterApi:Timeout", 30);
            client.BaseAddress = new Uri(baseUrl);
            client.Timeout = TimeSpan.FromSeconds(timeout);
        })
        .AddPolicyHandler(GetRetryPolicy())
        .AddPolicyHandler(GetCircuitBreakerPolicy())
        .AddPolicyHandler(GetTimeoutPolicy());

        services.AddHttpClient<ClickStreamService>("ClickAggregatorApi", client =>
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

        // Register Entity Framework services
        services.AddScoped<IRouteService, EfRouteService>();
        services.AddScoped<ICertificateService, EfCertificateService>();
        services.AddScoped<IUserSettingsService, EfUserSettingsService>();

        // Keep ClickStream as HTTP client proxy
        services.AddScoped<IClickStreamService, ClickAggregatorApiClient>();

        // Register HTTP client for Outbox background service
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

        // Register Outbox background service
        services.AddHostedService<OutboxProcessorService>();

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
            .OrResult(msg => !msg.IsSuccessStatusCode)
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
            .OrResult(msg => !msg.IsSuccessStatusCode)
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
        return Policy.TimeoutAsync<HttpResponseMessage>(TimeSpan.FromSeconds(10));
    }
}