variable "stage" {
  type = string
  default = "local"
}

variable "config_path" {
  type = string
  default = "~/.kube/config"
}

variable "config_context" {
  type = string
  default = "minikube"
}