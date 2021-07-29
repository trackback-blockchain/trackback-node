#!make

SHELL:=/bin/bash
.DEFAULT=all

export PROJECT_NAME					:= tanz-demo-node
export TARGET_PORT					:= 80
export REGION						:= ap-southeast-2
export ECR_REPO_URL					:= 533545012068.dkr.ecr.ap-southeast-2.amazonaws.com
export VERSION						:= latest
export BRANCH_NAME					:=$(shell git branch --show-current)

all: deploy

destroy:
	cd terraform/ap-southeast-2 && terraform destroy --auto-approve

ecr:
	aws ecr get-login-password \
    --region ${REGION} \
	| docker login \
		--username AWS \
		--password-stdin ${ECR_REPO_URL}

build: ecr
	docker build -f Dockerfile.dev --no-cache -t $(PROJECT_NAME):latest  .
	
	docker tag $(PROJECT_NAME):latest $(ECR_REPO_URL)/$(PROJECT_NAME):$(VERSION)
	docker push $(ECR_REPO_URL)/$(PROJECT_NAME):$(VERSION)

deploy:
	cd terraform/ap-southeast-2 && terraform destroy -var="branch_name=$(BRANCH_NAME)" --auto-approve 
	cd terraform/ap-southeast-2 && terraform apply -var="branch_name=$(BRANCH_NAME)" --auto-approve 

run-dev: ecr
	docker-compose -f docker-compose.dev.yml up --build --force-recreate --remove-orphans -d
