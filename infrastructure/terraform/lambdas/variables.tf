variable "name" {
  type        = string
  description = "Project name for Lambdas"
}

variable "apigw_arn" {
  type        = string
  description = "ARN of API Gateway that can access the Lambdas"
}
