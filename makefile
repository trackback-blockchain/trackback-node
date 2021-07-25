all:
	cd terraform/ap-southease-2 && terraform destroy --auto-approve
	cd terraform/ap-southease-2 && terraform apply --auto-approve

destroy:
	cd terraform/ap-southease-2 && terraform destroy --auto-approve
