resource "aws_lambda_permission" "apigw_lambda" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.recipes.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = var.apigw_arn
}

resource "aws_lambda_function" "recipes" {
  filename      = "../lambdas/out/recipes/lambda.zip"
  function_name = "${var.name}-recipes"
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
  name               = "${var.name}-lambda-recipes"
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

resource "aws_iam_policy_attachment" "dynamo" {
  name       = "${var.name}-lambda-recipes-dynamo-attachment"
  roles      = [aws_iam_role.iam_for_lambda.name]
  policy_arn = aws_iam_policy.dynamo.arn
}

resource "aws_iam_policy" "dynamo" {
  name        = "${var.name}-lambda-recipes-dynamo-policy"
  path        = "/"
  description = "Allows Lambdas access to read/write to DynamoDB"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "ReadWriteTable",
        Effect = "Allow",
        Action = [
          "dynamodb:BatchGetItem",
          "dynamodb:GetItem",
          "dynamodb:Query",
          "dynamodb:Scan",
          "dynamodb:BatchWriteItem",
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem"
        ],
        Resource = "arn:aws:dynamodb:*:*:table/*"
      }
    ]
  })
}
