data "aws_eip" "by_allocation_id" {
  id = "eipalloc-02762875079a54e2e"
}

resource "aws_security_group" "tanz_node" {
  name = "security_group for substrate node"

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



resource "aws_instance" "tanz_web" {
  ami                         = "ami-0567f647e75c7bc05"
  instance_type               = "t3.medium"
  vpc_security_group_ids      = [aws_security_group.tanz_node.id]
  associate_public_ip_address = false
  key_name                    = var.key_name

  tags = {
    Name = "tanz_web"
  }

  user_data = <<-EOF
#!/bin/bash
apt-get update
apt-get install -y apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   stable"
apt-get update
apt-get install -y docker-ce
chmod 666 /var/run/docker.sock
apt-get install -y git
usermod -aG docker ubuntu

# Install docker-compose
curl -L https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m) -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

cd /home/ubuntu

echo "run" > run
git clone --single-branch --branch staging https://${var.git_token}@github.com/trackback-blockchain/tanz-demo-node.git
cd tanz-demo-node
mkdir .local
docker-compose up --build --force-recreate --remove-orphans -d
EOF

}

resource "aws_eip_association" "eip_assoc" {
  instance_id   = aws_instance.tanz_web.id
  allocation_id = data.aws_eip.by_allocation_id.id
}

output "public_ip" {
  value = aws_instance.tanz_web
}

