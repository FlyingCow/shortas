using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class AddClickRouterComplianceFields : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_Routes_RouteProperties_PropertiesId",
                table: "Routes");

            migrationBuilder.DropTable(
                name: "ClickStreams");

            migrationBuilder.AlterColumn<long>(
                name: "Ttl",
                table: "Routes",
                type: "bigint",
                nullable: true,
                oldClrType: typeof(int),
                oldType: "integer");

            migrationBuilder.AlterColumn<string>(
                name: "Terminal",
                table: "Routes",
                type: "text",
                nullable: false,
                defaultValue: "External",
                oldClrType: typeof(string),
                oldType: "text");

            migrationBuilder.AlterColumn<string>(
                name: "Status",
                table: "Routes",
                type: "text",
                nullable: false,
                defaultValue: "Active",
                oldClrType: typeof(string),
                oldType: "text");

            migrationBuilder.AlterColumn<Guid>(
                name: "PropertiesId",
                table: "Routes",
                type: "uuid",
                nullable: false,
                defaultValue: new Guid("00000000-0000-0000-0000-000000000000"),
                oldClrType: typeof(Guid),
                oldType: "uuid",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "DestFormat",
                table: "Routes",
                type: "text",
                nullable: false,
                defaultValue: "Http",
                oldClrType: typeof(string),
                oldType: "text");

            migrationBuilder.AlterColumn<string>(
                name: "Dest",
                table: "Routes",
                type: "text",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "text");

            migrationBuilder.AlterColumn<int>(
                name: "Code",
                table: "Routes",
                type: "integer",
                nullable: true,
                oldClrType: typeof(int),
                oldType: "integer");

            migrationBuilder.AddColumn<string>(
                name: "PolicyJson",
                table: "Routes",
                type: "jsonb",
                nullable: false,
                defaultValue: "\"Basic\"");

            migrationBuilder.AlterColumn<string>(
                name: "RouteId",
                table: "RouteProperties",
                type: "character varying(255)",
                maxLength: 255,
                nullable: true,
                oldClrType: typeof(string),
                oldType: "character varying(255)",
                oldMaxLength: 255);

            migrationBuilder.AlterColumn<string>(
                name: "OwnerId",
                table: "RouteProperties",
                type: "character varying(255)",
                maxLength: 255,
                nullable: true,
                oldClrType: typeof(string),
                oldType: "text");

            migrationBuilder.AlterColumn<bool>(
                name: "Opengraph",
                table: "RouteProperties",
                type: "boolean",
                nullable: false,
                defaultValue: false,
                oldClrType: typeof(bool),
                oldType: "boolean");

            migrationBuilder.AlterColumn<string>(
                name: "DomainId",
                table: "RouteProperties",
                type: "character varying(255)",
                maxLength: 255,
                nullable: true,
                oldClrType: typeof(string),
                oldType: "text");

            migrationBuilder.AlterColumn<bool>(
                name: "AllowDebug",
                table: "RouteProperties",
                type: "boolean",
                nullable: false,
                defaultValue: false,
                oldClrType: typeof(bool),
                oldType: "boolean");

            migrationBuilder.AddColumn<string>(
                name: "BundlingJson",
                table: "RouteProperties",
                type: "jsonb",
                nullable: false,
                defaultValue: "{}");

            migrationBuilder.AddColumn<string>(
                name: "CreatorId",
                table: "RouteProperties",
                type: "character varying(255)",
                maxLength: 255,
                nullable: true);

            migrationBuilder.AddColumn<string>(
                name: "NativeJson",
                table: "RouteProperties",
                type: "jsonb",
                nullable: false,
                defaultValue: "{}");

            migrationBuilder.AddColumn<string>(
                name: "WorkspaceId",
                table: "RouteProperties",
                type: "character varying(255)",
                maxLength: 255,
                nullable: true);

            migrationBuilder.CreateIndex(
                name: "IX_RouteProperties_WorkspaceId",
                table: "RouteProperties",
                column: "WorkspaceId");

            migrationBuilder.AddForeignKey(
                name: "FK_Routes_RouteProperties_PropertiesId",
                table: "Routes",
                column: "PropertiesId",
                principalTable: "RouteProperties",
                principalColumn: "Id",
                onDelete: ReferentialAction.Cascade);
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_Routes_RouteProperties_PropertiesId",
                table: "Routes");

            migrationBuilder.DropIndex(
                name: "IX_RouteProperties_WorkspaceId",
                table: "RouteProperties");

            migrationBuilder.DropColumn(
                name: "PolicyJson",
                table: "Routes");

            migrationBuilder.DropColumn(
                name: "BundlingJson",
                table: "RouteProperties");

            migrationBuilder.DropColumn(
                name: "CreatorId",
                table: "RouteProperties");

            migrationBuilder.DropColumn(
                name: "NativeJson",
                table: "RouteProperties");

            migrationBuilder.DropColumn(
                name: "WorkspaceId",
                table: "RouteProperties");

            migrationBuilder.AlterColumn<int>(
                name: "Ttl",
                table: "Routes",
                type: "integer",
                nullable: false,
                defaultValue: 0,
                oldClrType: typeof(long),
                oldType: "bigint",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "Terminal",
                table: "Routes",
                type: "text",
                nullable: false,
                oldClrType: typeof(string),
                oldType: "text",
                oldDefaultValue: "External");

            migrationBuilder.AlterColumn<string>(
                name: "Status",
                table: "Routes",
                type: "text",
                nullable: false,
                oldClrType: typeof(string),
                oldType: "text",
                oldDefaultValue: "Active");

            migrationBuilder.AlterColumn<Guid>(
                name: "PropertiesId",
                table: "Routes",
                type: "uuid",
                nullable: true,
                oldClrType: typeof(Guid),
                oldType: "uuid");

            migrationBuilder.AlterColumn<string>(
                name: "DestFormat",
                table: "Routes",
                type: "text",
                nullable: false,
                oldClrType: typeof(string),
                oldType: "text",
                oldDefaultValue: "Http");

            migrationBuilder.AlterColumn<string>(
                name: "Dest",
                table: "Routes",
                type: "text",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "text",
                oldNullable: true);

            migrationBuilder.AlterColumn<int>(
                name: "Code",
                table: "Routes",
                type: "integer",
                nullable: false,
                defaultValue: 0,
                oldClrType: typeof(int),
                oldType: "integer",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "RouteId",
                table: "RouteProperties",
                type: "character varying(255)",
                maxLength: 255,
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "character varying(255)",
                oldMaxLength: 255,
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "OwnerId",
                table: "RouteProperties",
                type: "text",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "character varying(255)",
                oldMaxLength: 255,
                oldNullable: true);

            migrationBuilder.AlterColumn<bool>(
                name: "Opengraph",
                table: "RouteProperties",
                type: "boolean",
                nullable: false,
                oldClrType: typeof(bool),
                oldType: "boolean",
                oldDefaultValue: false);

            migrationBuilder.AlterColumn<string>(
                name: "DomainId",
                table: "RouteProperties",
                type: "text",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "character varying(255)",
                oldMaxLength: 255,
                oldNullable: true);

            migrationBuilder.AlterColumn<bool>(
                name: "AllowDebug",
                table: "RouteProperties",
                type: "boolean",
                nullable: false,
                oldClrType: typeof(bool),
                oldType: "boolean",
                oldDefaultValue: false);

            migrationBuilder.CreateTable(
                name: "ClickStreams",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Continent = table.Column<string>(type: "text", nullable: true),
                    Country = table.Column<string>(type: "text", nullable: true),
                    Created = table.Column<DateTime>(type: "timestamp with time zone", nullable: false),
                    CreatorId = table.Column<string>(type: "text", nullable: false),
                    Dest = table.Column<string>(type: "text", nullable: false),
                    DeviceBrand = table.Column<string>(type: "text", nullable: true),
                    DeviceFamily = table.Column<string>(type: "text", nullable: true),
                    DeviceModel = table.Column<string>(type: "text", nullable: true),
                    ExternalId = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    Ip = table.Column<string>(type: "text", nullable: false),
                    IsBot = table.Column<bool>(type: "boolean", nullable: false),
                    IsUnique = table.Column<bool>(type: "boolean", nullable: false),
                    Location = table.Column<string>(type: "text", nullable: true),
                    OsFamily = table.Column<string>(type: "text", nullable: true),
                    OsVersion = table.Column<string>(type: "text", nullable: true),
                    OwnerId = table.Column<string>(type: "text", nullable: false),
                    RouteId = table.Column<string>(type: "text", nullable: false),
                    SessionClicks = table.Column<long>(type: "bigint", nullable: true),
                    SessionFirst = table.Column<DateTime>(type: "timestamp with time zone", nullable: true),
                    UserAgentFamily = table.Column<string>(type: "text", nullable: true),
                    UserAgentVersion = table.Column<string>(type: "text", nullable: true),
                    WorkspaceId = table.Column<string>(type: "text", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_ClickStreams", x => x.Id);
                });

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

            migrationBuilder.AddForeignKey(
                name: "FK_Routes_RouteProperties_PropertiesId",
                table: "Routes",
                column: "PropertiesId",
                principalTable: "RouteProperties",
                principalColumn: "Id");
        }
    }
}
