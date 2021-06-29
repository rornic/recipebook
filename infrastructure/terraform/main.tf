provider "aws" {
  region = var.region
}

module "apigw" {
  source                    = "./apigw"
  name                      = var.name
  recipes_lambda_invoke_arn = module.lambdas.recipes_lambda_invoke_arn
  upload_lambda_invoke_arn  = module.lambdas.upload_lambda_invoke_arn
}

module "lambdas" {
  source    = "./lambdas"
  name      = var.name
  apigw_arn = module.apigw.apigw_arn
}

module "dynamo" {
  source = "./dynamo"
  name   = var.name
}

module "s3" {
  source = "./s3"
  name   = var.name
}
