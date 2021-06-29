resource "aws_lambda_permission" "apigw_lambda" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.upload.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = var.apigw_arn
}

resource "aws_lambda_function" "upload" {
  filename      = "../lambdas/out/upload/lambda.zip"
  function_name = "${var.name}-upload"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "doesnt.matter"

  runtime = "provided"
  environment {
    variables = {
      "RUST_BACKTRACE" = "1"
    }
  }
}

resource "aws_iam_role" "iam_for_lambda" {
  name               = "${var.name}-lambda-upload"
  assume_role_policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Action": "sts:AssumeRole",
            "Principal": {
                "Service": "lambda.amazonaws.com"
            },
            "Effect": "Allow",
            "Sid": ""
        }
    ]
}
  EOF
}

resource "aws_iam_policy_attachment" "s3" {
  name       = "s3-attachment"
  roles      = [aws_iam_role.iam_for_lambda.name]
  policy_arn = aws_iam_policy.s3.arn
}

resource "aws_iam_policy" "s3" {
  name        = "s3_policy"
  path        = "/"
  description = "Allows Lambdas access to read/write S3"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "ReadWriteS3",
        Effect = "Allow",
        Action = [
          "s3:PutObject",
          "s3:PutObjectAcl",
          "s3:GetObject",
          "s3:GetObjectAcl"
        ],
        Resource = "arn:aws:s3:::*/*"
      }
    ]
  })
}
