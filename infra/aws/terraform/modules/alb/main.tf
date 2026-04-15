# ALB Module for Shortas URL Shortener

# Public Application Load Balancer
resource "aws_lb" "public" {
  name               = "${var.environment}-shortas-public-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [var.public_alb_security_group_id]
  subnets            = var.public_subnet_ids

  enable_deletion_protection = var.environment == "prod"
  enable_http2               = true

  access_logs {
    bucket  = var.access_logs_bucket
    prefix  = "public-alb"
    enabled = var.enable_access_logs
  }

  tags = {
    Name        = "${var.environment}-shortas-public-alb"
    Environment = var.environment
  }
}

# Internal Application Load Balancer
resource "aws_lb" "internal" {
  name               = "${var.environment}-shortas-internal-alb"
  internal           = true
  load_balancer_type = "application"
  security_groups    = [var.internal_alb_security_group_id]
  subnets            = var.private_subnet_ids

  enable_deletion_protection = var.environment == "prod"
  enable_http2               = true

  tags = {
    Name        = "${var.environment}-shortas-internal-alb"
    Environment = var.environment
  }
}

# HTTPS Listener (Public ALB)
resource "aws_lb_listener" "https" {
  load_balancer_arn = aws_lb.public.arn
  port              = "443"
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-TLS13-1-2-2021-06"
  certificate_arn   = var.certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.click_router.arn
  }

  tags = {
    Name        = "${var.environment}-shortas-https-listener"
    Environment = var.environment
  }
}

# HTTP Listener - Redirect to HTTPS
resource "aws_lb_listener" "http" {
  load_balancer_arn = aws_lb.public.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "redirect"

    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }

  tags = {
    Name        = "${var.environment}-shortas-http-listener"
    Environment = var.environment
  }
}

# Internal HTTP Listener
resource "aws_lb_listener" "internal_http" {
  load_balancer_arn = aws_lb.internal.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "fixed-response"

    fixed_response {
      content_type = "text/plain"
      message_body = "Not Found"
      status_code  = "404"
    }
  }

  tags = {
    Name        = "${var.environment}-shortas-internal-http-listener"
    Environment = var.environment
  }
}

# Target Groups

# Click Router - Main redirect service (default)
resource "aws_lb_target_group" "click_router" {
  name        = "${var.environment}-click-router"
  port        = 5800
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200-399"
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-click-router-tg"
    Environment = var.environment
    Service     = "click-router"
  }
}

# Click Router API
resource "aws_lb_target_group" "click_router_api" {
  name        = "${var.environment}-click-router-api"
  port        = 5810
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-click-router-api-tg"
    Environment = var.environment
    Service     = "click-router-api"
  }
}

# Aggregator API
resource "aws_lb_target_group" "aggregator_api" {
  name        = "${var.environment}-aggregator-api"
  port        = 5820
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-aggregator-api-tg"
    Environment = var.environment
    Service     = "click-aggregator-api"
  }
}

# Shortas API (.NET)
resource "aws_lb_target_group" "shortas_api" {
  name        = "${var.environment}-shortas-api"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-shortas-api-tg"
    Environment = var.environment
    Service     = "shortas-api"
  }
}

# Dashboard (Frontend)
resource "aws_lb_target_group" "dashboard" {
  name        = "${var.environment}-dashboard"
  port        = 3000
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-dashboard-tg"
    Environment = var.environment
    Service     = "dashboard"
  }
}

# Landing Page
resource "aws_lb_target_group" "landing" {
  name        = "${var.environment}-landing"
  port        = 3000
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-landing-tg"
    Environment = var.environment
    Service     = "landing"
  }
}

# Pages Service
resource "aws_lb_target_group" "pages" {
  name        = "${var.environment}-pages"
  port        = 5801
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-pages-tg"
    Environment = var.environment
    Service     = "pages"
  }
}

# Keycloak (conditional - only created when using Keycloak instead of Cognito)
resource "aws_lb_target_group" "keycloak" {
  count = length(var.keycloak_host_headers) > 0 ? 1 : 0

  name        = "${var.environment}-keycloak"
  port        = 8080
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/health/ready"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  stickiness {
    type            = "lb_cookie"
    cookie_duration = 86400 # 1 day
    enabled         = true
  }

  tags = {
    Name        = "${var.environment}-keycloak-tg"
    Environment = var.environment
    Service     = "keycloak"
  }
}

# Domain Verifier
resource "aws_lb_target_group" "domain_verifier" {
  name        = "${var.environment}-domain-verifier"
  port        = 5830
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher             = "200"
    path                = "/health"
    port                = "traffic-port"
    protocol            = "HTTP"
    timeout             = 5
    unhealthy_threshold = 3
  }

  tags = {
    Name        = "${var.environment}-domain-verifier-tg"
    Environment = var.environment
    Service     = "domain-verifier"
  }
}

# Listener Rules for Path-Based Routing

# API routes -> Shortas API
resource "aws_lb_listener_rule" "api" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 10

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.shortas_api.arn
  }

  condition {
    path_pattern {
      values = ["/api/*"]
    }
  }

  condition {
    host_header {
      values = var.api_host_headers
    }
  }

  tags = {
    Name        = "${var.environment}-api-rule"
    Environment = var.environment
  }
}

# Dashboard routes
resource "aws_lb_listener_rule" "dashboard" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 20

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.dashboard.arn
  }

  condition {
    host_header {
      values = var.dashboard_host_headers
    }
  }

  tags = {
    Name        = "${var.environment}-dashboard-rule"
    Environment = var.environment
  }
}

# Landing page routes
resource "aws_lb_listener_rule" "landing" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 30

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.landing.arn
  }

  condition {
    host_header {
      values = var.landing_host_headers
    }
  }

  tags = {
    Name        = "${var.environment}-landing-rule"
    Environment = var.environment
  }
}

# Keycloak auth routes (conditional - only created when using Keycloak instead of Cognito)
resource "aws_lb_listener_rule" "keycloak" {
  count = length(var.keycloak_host_headers) > 0 ? 1 : 0

  listener_arn = aws_lb_listener.https.arn
  priority     = 40

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.keycloak[0].arn
  }

  condition {
    host_header {
      values = var.keycloak_host_headers
    }
  }

  tags = {
    Name        = "${var.environment}-keycloak-rule"
    Environment = var.environment
  }
}

# Router API routes
resource "aws_lb_listener_rule" "router_api" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 50

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.click_router_api.arn
  }

  condition {
    path_pattern {
      values = ["/router-api/*"]
    }
  }

  tags = {
    Name        = "${var.environment}-router-api-rule"
    Environment = var.environment
  }
}

# Aggregator API routes
resource "aws_lb_listener_rule" "aggregator_api" {
  listener_arn = aws_lb_listener.https.arn
  priority     = 60

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.aggregator_api.arn
  }

  condition {
    path_pattern {
      values = ["/analytics-api/*"]
    }
  }

  tags = {
    Name        = "${var.environment}-aggregator-api-rule"
    Environment = var.environment
  }
}

# Internal Listener Rules

# Pages service (internal)
resource "aws_lb_listener_rule" "internal_pages" {
  listener_arn = aws_lb_listener.internal_http.arn
  priority     = 10

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.pages.arn
  }

  condition {
    host_header {
      values = ["pages.${var.environment}.shortas.local"]
    }
  }

  tags = {
    Name        = "${var.environment}-internal-pages-rule"
    Environment = var.environment
  }
}

# Domain verifier (internal)
resource "aws_lb_listener_rule" "internal_domain_verifier" {
  listener_arn = aws_lb_listener.internal_http.arn
  priority     = 20

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.domain_verifier.arn
  }

  condition {
    host_header {
      values = ["domain-verifier.${var.environment}.shortas.local"]
    }
  }

  tags = {
    Name        = "${var.environment}-internal-domain-verifier-rule"
    Environment = var.environment
  }
}
