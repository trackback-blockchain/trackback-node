resource "aws_cloudwatch_log_group" "aws_cwl_ec2" {
  name =  var.cloud_watch_name
}