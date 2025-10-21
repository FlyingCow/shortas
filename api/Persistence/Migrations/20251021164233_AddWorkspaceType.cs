using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace ShortasProxyApi.Persistence.Migrations
{
    /// <inheritdoc />
    public partial class AddWorkspaceType : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AddColumn<string>(
                name: "Type",
                table: "Workspaces",
                type: "character varying(50)",
                maxLength: 50,
                nullable: false,
                defaultValue: "User");

            migrationBuilder.CreateIndex(
                name: "IX_Workspaces_Type",
                table: "Workspaces",
                column: "Type");
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropIndex(
                name: "IX_Workspaces_Type",
                table: "Workspaces");

            migrationBuilder.DropColumn(
                name: "Type",
                table: "Workspaces");
        }
    }
}
