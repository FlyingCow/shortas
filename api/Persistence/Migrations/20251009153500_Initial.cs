using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class Initial : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "Certificates",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Key = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    Cert = table.Column<string>(type: "text", nullable: false),
                    OcspResp = table.Column<string>(type: "text", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_Certificates", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "ClickStreams",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    ExternalId = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    OwnerId = table.Column<string>(type: "text", nullable: false),
                    CreatorId = table.Column<string>(type: "text", nullable: false),
                    RouteId = table.Column<string>(type: "text", nullable: false),
                    WorkspaceId = table.Column<string>(type: "text", nullable: false),
                    Created = table.Column<DateTime>(type: "timestamp with time zone", nullable: false),
                    Dest = table.Column<string>(type: "text", nullable: false),
                    Ip = table.Column<string>(type: "text", nullable: false),
                    Continent = table.Column<string>(type: "text", nullable: true),
                    Country = table.Column<string>(type: "text", nullable: true),
                    Location = table.Column<string>(type: "text", nullable: true),
                    OsFamily = table.Column<string>(type: "text", nullable: true),
                    OsVersion = table.Column<string>(type: "text", nullable: true),
                    UserAgentFamily = table.Column<string>(type: "text", nullable: true),
                    UserAgentVersion = table.Column<string>(type: "text", nullable: true),
                    DeviceBrand = table.Column<string>(type: "text", nullable: true),
                    DeviceFamily = table.Column<string>(type: "text", nullable: true),
                    DeviceModel = table.Column<string>(type: "text", nullable: true),
                    SessionFirst = table.Column<DateTime>(type: "timestamp with time zone", nullable: true),
                    SessionClicks = table.Column<long>(type: "bigint", nullable: true),
                    IsUnique = table.Column<bool>(type: "boolean", nullable: false),
                    IsBot = table.Column<bool>(type: "boolean", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_ClickStreams", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "OutboxMessages",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    EventType = table.Column<string>(type: "character varying(50)", maxLength: 50, nullable: false),
                    AggregateId = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    Payload = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "{}"),
                    CreatedAt = table.Column<DateTime>(type: "timestamp with time zone", nullable: false),
                    ProcessedAt = table.Column<DateTime>(type: "timestamp with time zone", nullable: true),
                    Status = table.Column<string>(type: "character varying(20)", maxLength: 20, nullable: false, defaultValue: "Pending"),
                    RetryCount = table.Column<int>(type: "integer", nullable: false, defaultValue: 0),
                    MaxRetries = table.Column<int>(type: "integer", nullable: false, defaultValue: 5),
                    ErrorMessage = table.Column<string>(type: "character varying(2000)", maxLength: 2000, nullable: true),
                    NextRetryAt = table.Column<DateTime>(type: "timestamp with time zone", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_OutboxMessages", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "RouteProperties",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    RouteId = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    DomainId = table.Column<string>(type: "text", nullable: false),
                    OwnerId = table.Column<string>(type: "text", nullable: false),
                    ScriptsJson = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "[]"),
                    TagsJson = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "[]"),
                    CustomJson = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "{}"),
                    Opengraph = table.Column<bool>(type: "boolean", nullable: false),
                    AllowDebug = table.Column<bool>(type: "boolean", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_RouteProperties", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "UserSettings",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Email = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    Status = table.Column<string>(type: "text", nullable: false),
                    Debug = table.Column<bool>(type: "boolean", nullable: false),
                    Overflow = table.Column<bool>(type: "boolean", nullable: false),
                    SkipTrackingJson = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "[]"),
                    AllowedRequestParamsJson = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "[]"),
                    AllowedDestinationParamsJson = table.Column<string>(type: "jsonb", nullable: false, defaultValue: "[]")
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_UserSettings", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "Routes",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Switch = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    Link = table.Column<string>(type: "text", nullable: false),
                    Dest = table.Column<string>(type: "text", nullable: false),
                    DestFormat = table.Column<string>(type: "text", nullable: false),
                    Code = table.Column<int>(type: "integer", nullable: false),
                    Ttl = table.Column<int>(type: "integer", nullable: false),
                    Status = table.Column<string>(type: "text", nullable: false),
                    Terminal = table.Column<string>(type: "text", nullable: false),
                    PropertiesId = table.Column<Guid>(type: "uuid", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_Routes", x => x.Id);
                    table.ForeignKey(
                        name: "FK_Routes_RouteProperties_PropertiesId",
                        column: x => x.PropertiesId,
                        principalTable: "RouteProperties",
                        principalColumn: "Id");
                });

            migrationBuilder.CreateIndex(
                name: "IX_Certificates_Key",
                table: "Certificates",
                column: "Key");

            migrationBuilder.CreateIndex(
                name: "IX_ClickStreams_Created",
                table: "ClickStreams",
                column: "Created");

            migrationBuilder.CreateIndex(
                name: "IX_ClickStreams_ExternalId",
                table: "ClickStreams",
                column: "ExternalId");

            migrationBuilder.CreateIndex(
                name: "IX_ClickStreams_OwnerId",
                table: "ClickStreams",
                column: "OwnerId");

            migrationBuilder.CreateIndex(
                name: "IX_ClickStreams_RouteId",
                table: "ClickStreams",
                column: "RouteId");

            migrationBuilder.CreateIndex(
                name: "IX_OutboxMessages_CreatedAt",
                table: "OutboxMessages",
                column: "CreatedAt");

            migrationBuilder.CreateIndex(
                name: "IX_OutboxMessages_Status",
                table: "OutboxMessages",
                column: "Status");

            migrationBuilder.CreateIndex(
                name: "IX_OutboxMessages_Status_NextRetryAt",
                table: "OutboxMessages",
                columns: new[] { "Status", "NextRetryAt" });

            migrationBuilder.CreateIndex(
                name: "IX_RouteProperties_OwnerId",
                table: "RouteProperties",
                column: "OwnerId");

            migrationBuilder.CreateIndex(
                name: "IX_RouteProperties_RouteId",
                table: "RouteProperties",
                column: "RouteId");

            migrationBuilder.CreateIndex(
                name: "IX_Routes_Link",
                table: "Routes",
                column: "Link");

            migrationBuilder.CreateIndex(
                name: "IX_Routes_PropertiesId",
                table: "Routes",
                column: "PropertiesId");

            migrationBuilder.CreateIndex(
                name: "IX_Routes_Status",
                table: "Routes",
                column: "Status");

            migrationBuilder.CreateIndex(
                name: "IX_UserSettings_Email",
                table: "UserSettings",
                column: "Email",
                unique: true);
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "Certificates");

            migrationBuilder.DropTable(
                name: "ClickStreams");

            migrationBuilder.DropTable(
                name: "OutboxMessages");

            migrationBuilder.DropTable(
                name: "Routes");

            migrationBuilder.DropTable(
                name: "UserSettings");

            migrationBuilder.DropTable(
                name: "RouteProperties");
        }
    }
}
