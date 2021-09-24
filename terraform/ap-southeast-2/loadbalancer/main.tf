module "dev-nodes" {
  source             = "../../modules/loadbalancer_r53"
  certificate_arn    = var.certificate_arn
  load_balancer_name = var.load_balancer_name
  zone_id            = var.zone_id
  domain             = var.domain

}

output "info" {
  value = module.dev-nodes
}
