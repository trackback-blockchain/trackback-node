all:
	cd terraform/ap-southeast-2 && terraform destroy --auto-approve
	cd terraform/ap-southeast-2 && terraform apply --auto-approve

destroy:
	cd terraform/ap-southeast-2 && terraform destroy --auto-approve
