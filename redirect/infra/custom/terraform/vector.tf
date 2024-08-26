resource "helm_release" "vector" {
  name      = "vector"
  namespace = "shortas-core"

  repository = "https://helm.vector.dev"
  chart      = "vector"
}