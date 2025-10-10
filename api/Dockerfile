# Use the official .NET 8 runtime as the base image
FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
WORKDIR /app
EXPOSE 8080
EXPOSE 8081

# Use the official .NET 8 SDK image for building
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY ["ShortasProxyApi.csproj", "."]
RUN dotnet restore "ShortasProxyApi.csproj"
COPY . .
WORKDIR "/src"
RUN dotnet build "ShortasProxyApi.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "ShortasProxyApi.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .

# Create logs directory
RUN mkdir -p /app/logs

# Set environment variables
ENV ASPNETCORE_URLS=http://+:8080
ENV ASPNETCORE_ENVIRONMENT=Production

ENTRYPOINT ["dotnet", "ShortasProxyApi.dll"]

