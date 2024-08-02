resource "kubernetes_namespace" "shortas_core" {
  metadata {
    name = "shortas-core"
  }
}