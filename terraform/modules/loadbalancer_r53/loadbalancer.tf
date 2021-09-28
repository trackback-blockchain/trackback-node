
locals {
  targetGroupSettings = {
    "9944" : { port : "9944", listenerPort : "443" },
    "9933" : { port : "9933", listenerPort : "9933" },
    "30333" : { port : "30333", listenerPort : "30333" },
  }

}

resource "aws_security_group" "aws_sg_lb" {
  name = "${var.load_balancer_name} SG: 22 80 9944 9933 30333"

  ingress {
    description = "SSH from the internet"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "80 from the internet"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "9944 from the internet"
    from_port   = 9944
    to_port     = 9944
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "9933 from the internet"
    from_port   = 9933
    to_port     = 9933
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "30333 from the internet"
    from_port   = 30333
    to_port     = 30333
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

}


resource "aws_lb" "main" {
  name = var.load_balancer_name

  internal           = false
  load_balancer_type = "application"
  subnets            = ["subnet-ea47828c", "subnet-ece91fa4", "subnet-cd384f95"]

  security_groups = [aws_security_group.aws_sg_lb.id]
}

resource "aws_lb_target_group" "targetGroups" {

  for_each = local.targetGroupSettings
  name     = "${var.load_balancer_name}TG${each.value.port}"
  port     = each.value.port
  protocol = "HTTP"
  vpc_id   = "vpc-fa9f829d"

  health_check {
    enabled = true
    port    = 80
    path    = "/"
  }
}


resource "aws_lb_listener" "listners" {

  for_each = {
    for tg in aws_lb_target_group.targetGroups : tg.name => {
      arn : tg.arn
      port : tg.port == 9944 ? 443 : tg.port
    }
  }

  load_balancer_arn = aws_lb.main.arn
  port              = each.value.port
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = var.certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = each.value.arn
  }
}

output "targetGroups" {
  value = {
    for tg in aws_lb_target_group.targetGroups : tg.name => {
      arn : tg.arn
    }
  }
}
