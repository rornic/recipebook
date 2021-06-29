variable "name" {
  type        = string
  description = "Name of the API Gateway"
}

variable "recipes_lambda_invoke_arn" {
  type        = string
  description = "Invoke ARN of recipes lambda"
}

variable "upload_lambda_invoke_arn" {
  type        = string
  description = "Invoke ARN of upload lambda"
}
