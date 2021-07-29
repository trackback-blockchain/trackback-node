module "demo-infra" {
  source = "../modules/ec2_docker"
  branch_name = var.branch_name
}

output "info" {
  value = module.demo-infra
}