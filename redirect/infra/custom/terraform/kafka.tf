resource "helm_release" "kafka" {
  name      = "kafka"
  namespace = "shortas-core"

  repository = "https://charts.bitnami.com/bitnami"
  chart      = "kafka"

  set {
    name  = "provisioning.enabled"
    value = "true"
  }

  set {
    name  = "provisioning.topics[0].name"
    value = "hit-stream-${var.stage}"
  }

  set {
    name  = "provisioning.topics[0].partitions"
    value = "2"
  }
}
