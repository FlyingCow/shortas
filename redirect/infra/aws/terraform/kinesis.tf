resource "aws_kinesis_stream" "hit-stream" {
  name             = "hit-stream-${var.stage}"
  shard_count      = 2
  retention_period = 48

  shard_level_metrics = [
    "IncomingBytes",
    "OutgoingBytes",
  ]

  stream_mode_details {
    stream_mode = "PROVISIONED"
  }

  tags = {
    Environment = "${var.stage}"
  }
}