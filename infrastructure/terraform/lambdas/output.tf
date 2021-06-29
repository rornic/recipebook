output "recipes_lambda_invoke_arn" {
  value       = module.recipes.invoke_arn
  description = "Invoke ARN of recipes lambda"
}

output "upload_lambda_invoke_arn" {
  value       = module.upload.invoke_arn
  description = "Invoke ARN of upload lambda"
}
