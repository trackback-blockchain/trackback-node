data "aws_lb_target_group" "tg_substrateNode" {
  arn = "arn:aws:elasticloadbalancing:ap-southeast-2:533545012068:targetgroup/SubstrateNode/0314959edf168f21"
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



resource "aws_instance" "tanz_demo_web" {
  ami                         = "ami-0567f647e75c7bc05"
  instance_type               = "c4.xlarge"
  vpc_security_group_ids      = [aws_security_group.tanz_node.id]
  associate_public_ip_address = false
  key_name                    = var.key_name
  iam_instance_profile        = aws_iam_instance_profile.tz-demo-profile.id

  tags = {
    Name = "TrackBack-Node"
  }

  root_block_device {
    volume_type = "gp2"
    volume_size = 100
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

curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
apt install -y unzip
unzip awscliv2.zip
sudo ./aws/install
apt install -y make

git clone --single-branch --branch ${var.branch_name} https://${var.git_token}@github.com/trackback-blockchain/trackback-node.git repo
chown ubuntu:ubuntu -R repo

cd repo
make run-dev

EOF

}

resource "aws_lb_target_group_attachment" "tg_attachment" {
  target_group_arn = data.aws_lb_target_group.tg_substrateNode.arn
  target_id        = aws_instance.tanz_demo_web.id
  port             = 9944
}

output "tanz_demo_web" {
  value = aws_instance.tanz_demo_web
}

