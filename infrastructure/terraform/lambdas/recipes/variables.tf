variable "name" {
  type        = string
  description = "Project name to use for this Lambda"
}

variable "apigw_arn" {
  type        = string
  description = "ARN of API Gateway that executes this Lambda"
}
