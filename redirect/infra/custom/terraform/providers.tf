terraform {
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.31.0"
    }
    kafka = {
      source = "Mongey/kafka"
    }
  }
}

provider "kubernetes" {
  config_path    = var.config_path
  config_context = var.config_context
}

provider "kafka" {
  bootstrap_servers = ["localhost:9092"]
}