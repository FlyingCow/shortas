#!/bin/bash
set -e

echo "Applying EF Core migrations..."
./efbundle --connection "$ConnectionStrings__DefaultConnection"
echo "Migrations applied successfully."

exec dotnet ShortasProxyApi.dll
