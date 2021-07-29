data "aws_iam_policy_document" "tz-demo-role-ecr" {
  statement {
    sid    = ""
    effect = "Allow"

    resources = ["*"]

    actions = [
      "ecr:GetAuthorizationToken",
      "ecr:BatchCheckLayerAvailability",
      "ecr:GetDownloadUrlForLayer",
      "ecr:GetRepositoryPolicy",
      "ecr:DescribeRepositories",
      "ecr:ListImages",
      "ecr:DescribeImages",
      "ecr:BatchGetImage",
      "ecr:GetLifecyclePolicy",
      "ecr:GetLifecyclePolicyPreview",
      "ecr:ListTagsForResource",
      "ecr:DescribeImageScanFindings"
    ]
  }
}

data "aws_iam_policy_document" "tz-demo-assume-role-policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "tz-demo-role" {
  name = "tz-demo-role"

  assume_role_policy = data.aws_iam_policy_document.tz-demo-assume-role-policy.json
}

resource "aws_iam_instance_profile" "tz-demo-profile" {
  name = "tz-demo-profile"
  role = aws_iam_role.tz-demo-role.id
}

resource "aws_iam_role_policy" "tz-demo-role_policy" {
  name = "tz-demo-role_policy"
  role = aws_iam_role.tz-demo-role.id

  policy = data.aws_iam_policy_document.tz-demo-role-ecr.json
}
