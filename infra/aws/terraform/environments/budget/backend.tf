# Terraform Backend Configuration for Budget
# Uncomment and configure after creating the S3 bucket

# terraform {
#   backend "s3" {
#     bucket         = "shortas-terraform-state-budget"
#     key            = "budget/terraform.tfstate"
#     region         = "us-east-1"
#     encrypt        = true
#     dynamodb_table = "shortas-terraform-locks-budget"
#   }
# }
