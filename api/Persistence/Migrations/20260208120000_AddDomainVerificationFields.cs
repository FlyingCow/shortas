using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class AddDomainVerificationFields : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AddColumn<int>(
                name: "VerificationStatus",
                table: "Domains",
                type: "integer",
                nullable: false,
                defaultValue: 0);

            migrationBuilder.AddColumn<string>(
                name: "VerificationReason",
                table: "Domains",
                type: "character varying(255)",
                maxLength: 255,
                nullable: false,
                defaultValue: "not_checked");

            migrationBuilder.AddColumn<DateTime>(
                name: "LastVerificationCheck",
                table: "Domains",
                type: "timestamp with time zone",
                nullable: true);

            migrationBuilder.AddColumn<DateTime>(
                name: "NextVerificationCheck",
                table: "Domains",
                type: "timestamp with time zone",
                nullable: true);

            migrationBuilder.CreateIndex(
                name: "IX_Domains_VerificationStatus",
                table: "Domains",
                column: "VerificationStatus");
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropIndex(
                name: "IX_Domains_VerificationStatus",
                table: "Domains");

            migrationBuilder.DropColumn(
                name: "VerificationStatus",
                table: "Domains");

            migrationBuilder.DropColumn(
                name: "VerificationReason",
                table: "Domains");

            migrationBuilder.DropColumn(
                name: "LastVerificationCheck",
                table: "Domains");

            migrationBuilder.DropColumn(
                name: "NextVerificationCheck",
                table: "Domains");
        }
    }
}
