module "recipes" {
  source    = "./recipes"
  name      = var.name
  apigw_arn = var.apigw_arn
}

module "upload" {
  source    = "./upload"
  name      = var.name
  apigw_arn = var.apigw_arn
}

