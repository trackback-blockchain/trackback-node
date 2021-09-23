module "demo-infra" {
  source           = "../modules/ec2_docker"
  branch_name      = var.branch_name
  cloud_watch_name = var.cloud_watch_name
}

output "info" {
  value = module.demo-infra
}