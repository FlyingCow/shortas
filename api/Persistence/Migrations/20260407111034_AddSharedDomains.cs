using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class AddSharedDomains : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropIndex(
                name: "IX_Domains_OwnerId_Name",
                table: "Domains");

            migrationBuilder.AlterColumn<string>(
                name: "CustomNotFoundUrl",
                table: "Domains",
                type: "text",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "character varying(2048)",
                oldMaxLength: 2048,
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "CustomIndexUrl",
                table: "Domains",
                type: "text",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "character varying(2048)",
                oldMaxLength: 2048,
                oldNullable: true);

            migrationBuilder.AddColumn<bool>(
                name: "IsShared",
                table: "Domains",
                type: "boolean",
                nullable: false,
                defaultValue: false);

            migrationBuilder.CreateIndex(
                name: "IX_Domains_Name",
                table: "Domains",
                column: "Name",
                unique: true);
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropIndex(
                name: "IX_Domains_Name",
                table: "Domains");

            migrationBuilder.DropColumn(
                name: "IsShared",
                table: "Domains");

            migrationBuilder.AlterColumn<string>(
                name: "CustomNotFoundUrl",
                table: "Domains",
                type: "character varying(2048)",
                maxLength: 2048,
                nullable: true,
                oldClrType: typeof(string),
                oldType: "text",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "CustomIndexUrl",
                table: "Domains",
                type: "character varying(2048)",
                maxLength: 2048,
                nullable: true,
                oldClrType: typeof(string),
                oldType: "text",
                oldNullable: true);

            migrationBuilder.CreateIndex(
                name: "IX_Domains_OwnerId_Name",
                table: "Domains",
                columns: new[] { "OwnerId", "Name" },
                unique: true);
        }
    }
}
