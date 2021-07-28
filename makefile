#!make

SHELL:=/bin/bash
.DEFAULT=all

export PROJECT_NAME					:= tanz-demo-node
export TARGET_PORT					:= 80
export REGION						:= ap-southeast-2
export ECR_REPO_URL					:= 533545012068.dkr.ecr.ap-southeast-2.amazonaws.com
export VERSION						:= latest

all:
	cd terraform/ap-southeast-2 && terraform destroy --auto-approve
	cd terraform/ap-southeast-2 && terraform apply --auto-approve

destroy:
	cd terraform/ap-southeast-2 && terraform destroy --auto-approve


build:
	aws ecr get-login-password \
    --region ${REGION} \
	| docker login \
		--username AWS \
		--password-stdin ${ECR_REPO_URL}

	-aws ecr create-repository \
    --repository-name $(PROJECT_NAME) \
    --image-scanning-configuration scanOnPush=true \
    --region ${REGION} > /dev/null

	docker build -f Dockerfile.dev --no-cache -t $(PROJECT_NAME):latest  .
	
	docker tag $(PROJECT_NAME):latest $(ECR_REPO_URL)/$(PROJECT_NAME):$(VERSION)
	docker push $(ECR_REPO_URL)/$(PROJECT_NAME):$(VERSION)

deploy:
	cd terraform/ap-southeast-2 && terraform destroy --auto-approve
	cd terraform/ap-southeast-2 && terraform apply --auto-approve