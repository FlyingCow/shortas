using ShortasProxyApi.Application.Services;

namespace ShortasProxyApi.Application;

public static class ServiceCollectionExtensions
{
    public static IServiceCollection AddApplicationServices(this IServiceCollection services)
    {
        // Register application services
        services.AddScoped<RouteService>();
        services.AddScoped<CertificateService>();
        services.AddScoped<UserSettingsService>();

        return services;
    }
}

