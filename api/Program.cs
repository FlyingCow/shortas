using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Infrastructure;
using ShortasProxyApi.Application;
using ShortasProxyApi.Presentation;
using ShortasProxyApi.Infrastructure.Security;
using Serilog;
using ShortasProxyApi.Infrastructure.Data;

var builder = WebApplication.CreateBuilder(args);

// Configure Serilog
Log.Logger = new LoggerConfiguration()
    .ReadFrom.Configuration(builder.Configuration)
    .Enrich.FromLogContext()
    .WriteTo.Console()
    .WriteTo.File("logs/shortas-api-.txt", rollingInterval: RollingInterval.Day)
    .CreateLogger();

builder.Host.UseSerilog();

// Add services to the container
builder.Services.AddApplicationServices();
builder.Services.AddInfrastructureServices(builder.Configuration);
builder.Services.AddPresentationServices();

// Add CORS
builder.Services.AddCors(options =>
{
    options.AddPolicy("DashboardPolicy", policy =>
    {
        var allowedOrigins = builder.Configuration.GetSection("Security:AllowedOrigins").Get<string[]>()
                             ?? new[] { "http://localhost:3000", "https://localhost:3000" };

        policy.WithOrigins(allowedOrigins)
            .AllowAnyHeader()
            .AllowAnyMethod()
            .AllowCredentials();
    });
});

var app = builder.Build();

// Configure the HTTP request pipeline
// Enable Swagger first (before any other middleware)
app.UseSwagger();
app.UseSwaggerUI(c =>
{
    c.SwaggerEndpoint("/swagger/v1/swagger.json", "Shortas Proxy API v1");
    c.RoutePrefix = "swagger"; // Set Swagger UI at /swagger
});

app.UseHttpsRedirection();

// Add CORS
app.UseCors("DashboardPolicy");

// Add security headers
app.UseMiddleware<SecurityHeadersMiddleware>();

// Add rate limiting
app.UseMiddleware<RateLimitingMiddleware>();

// Add request logging
app.UseSerilogRequestLogging();

app.UseAuthentication();
app.UseAuthorization();

app.MapControllers();

app.Run("http://localhost:5050");