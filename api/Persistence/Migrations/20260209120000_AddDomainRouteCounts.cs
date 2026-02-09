using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class AddDomainRouteCounts : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "DomainRouteCounts",
                columns: table => new
                {
                    DomainId = table.Column<Guid>(type: "uuid", nullable: false),
                    RouteCount = table.Column<int>(type: "integer", nullable: false, defaultValue: 0)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_DomainRouteCounts", x => x.DomainId);
                    table.ForeignKey(
                        name: "FK_DomainRouteCounts_Domains_DomainId",
                        column: x => x.DomainId,
                        principalTable: "Domains",
                        principalColumn: "Id",
                        onDelete: ReferentialAction.Cascade);
                });

            // Backfill existing counts from the Routes table
            migrationBuilder.Sql(@"
                INSERT INTO ""DomainRouteCounts"" (""DomainId"", ""RouteCount"")
                SELECT ""DomainId"", COUNT(*)
                FROM ""Routes""
                WHERE ""DomainId"" IS NOT NULL
                GROUP BY ""DomainId""
            ");
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "DomainRouteCounts");
        }
    }
}
