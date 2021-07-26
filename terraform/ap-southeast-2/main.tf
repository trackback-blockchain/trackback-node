module "demo-infra" {
  source = "../modules/ec2_docker"
}

output "info" {
  value = module.demo-infra
}