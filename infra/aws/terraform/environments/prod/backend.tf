# Terraform Backend Configuration for Production
# Uncomment and configure after creating the S3 bucket

# terraform {
#   backend "s3" {
#     bucket         = "shortas-terraform-state-prod"
#     key            = "prod/terraform.tfstate"
#     region         = "us-east-1"
#     encrypt        = true
#     dynamodb_table = "shortas-terraform-locks-prod"
#   }
# }
