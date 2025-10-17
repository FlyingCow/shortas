using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class RouteDomains : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AddColumn<Guid>(
                name: "DomainId",
                table: "Routes",
                type: "uuid",
                nullable: true);

            migrationBuilder.AddColumn<Guid>(
                name: "DomainId",
                table: "Certificates",
                type: "uuid",
                nullable: false,
                defaultValue: new Guid("00000000-0000-0000-0000-000000000000"));

            migrationBuilder.AddColumn<string>(
                name: "OwnerId",
                table: "Certificates",
                type: "character varying(255)",
                maxLength: 255,
                nullable: false,
                defaultValue: "");

            migrationBuilder.CreateTable(
                name: "Domains",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Name = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    OwnerId = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_Domains", x => x.Id);
                });

            migrationBuilder.CreateIndex(
                name: "IX_Routes_DomainId",
                table: "Routes",
                column: "DomainId");

            migrationBuilder.CreateIndex(
                name: "IX_Certificates_DomainId",
                table: "Certificates",
                column: "DomainId");

            migrationBuilder.CreateIndex(
                name: "IX_Certificates_OwnerId",
                table: "Certificates",
                column: "OwnerId");

            migrationBuilder.CreateIndex(
                name: "IX_Certificates_OwnerId_DomainId",
                table: "Certificates",
                columns: new[] { "OwnerId", "DomainId" });

            migrationBuilder.CreateIndex(
                name: "IX_Domains_OwnerId",
                table: "Domains",
                column: "OwnerId");

            migrationBuilder.CreateIndex(
                name: "IX_Domains_OwnerId_Name",
                table: "Domains",
                columns: new[] { "OwnerId", "Name" },
                unique: true);

            migrationBuilder.AddForeignKey(
                name: "FK_Certificates_Domains_DomainId",
                table: "Certificates",
                column: "DomainId",
                principalTable: "Domains",
                principalColumn: "Id",
                onDelete: ReferentialAction.Restrict);

            migrationBuilder.AddForeignKey(
                name: "FK_Routes_Domains_DomainId",
                table: "Routes",
                column: "DomainId",
                principalTable: "Domains",
                principalColumn: "Id",
                onDelete: ReferentialAction.Restrict);
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_Certificates_Domains_DomainId",
                table: "Certificates");

            migrationBuilder.DropForeignKey(
                name: "FK_Routes_Domains_DomainId",
                table: "Routes");

            migrationBuilder.DropTable(
                name: "Domains");

            migrationBuilder.DropIndex(
                name: "IX_Routes_DomainId",
                table: "Routes");

            migrationBuilder.DropIndex(
                name: "IX_Certificates_DomainId",
                table: "Certificates");

            migrationBuilder.DropIndex(
                name: "IX_Certificates_OwnerId",
                table: "Certificates");

            migrationBuilder.DropIndex(
                name: "IX_Certificates_OwnerId_DomainId",
                table: "Certificates");

            migrationBuilder.DropColumn(
                name: "DomainId",
                table: "Routes");

            migrationBuilder.DropColumn(
                name: "DomainId",
                table: "Certificates");

            migrationBuilder.DropColumn(
                name: "OwnerId",
                table: "Certificates");
        }
    }
}
