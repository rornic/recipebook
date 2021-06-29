
resource "aws_api_gateway_rest_api" "apigw" {
  name               = var.name
  binary_media_types = ["multipart/form-data"]
}


#########################
###### Enable CORS ######
#########################
module "recipes-cors" {
  source          = "squidfunk/api-gateway-enable-cors/aws"
  version         = "0.3.1"
  api_id          = aws_api_gateway_rest_api.apigw.id
  api_resource_id = aws_api_gateway_resource.recipes.id
}

module "recipe-cors" {
  source          = "squidfunk/api-gateway-enable-cors/aws"
  version         = "0.3.1"
  api_id          = aws_api_gateway_rest_api.apigw.id
  api_resource_id = aws_api_gateway_resource.recipe.id
}

module "upload-cors" {
  source          = "squidfunk/api-gateway-enable-cors/aws"
  version         = "0.3.1"
  api_id          = aws_api_gateway_rest_api.apigw.id
  api_resource_id = aws_api_gateway_resource.upload.id
}


###################
##### /upload #####
###################
resource "aws_api_gateway_resource" "upload" {
  parent_id   = aws_api_gateway_rest_api.apigw.root_resource_id
  path_part   = "upload"
  rest_api_id = aws_api_gateway_rest_api.apigw.id
}

resource "aws_api_gateway_method" "upload_post" {
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  authorization = "NONE"
  http_method   = "POST"
  resource_id   = aws_api_gateway_resource.upload.id
}

resource "aws_api_gateway_integration" "upload_post" {
  http_method             = aws_api_gateway_method.upload_post.http_method
  integration_http_method = "POST"
  resource_id             = aws_api_gateway_resource.upload.id
  rest_api_id             = aws_api_gateway_rest_api.apigw.id
  type                    = "AWS_PROXY"
  uri                     = var.upload_lambda_invoke_arn
}

######################
###### /recipes ######
######################
resource "aws_api_gateway_resource" "recipes" {
  parent_id   = aws_api_gateway_rest_api.apigw.root_resource_id
  path_part   = "recipes"
  rest_api_id = aws_api_gateway_rest_api.apigw.id
}

###############
##### GET #####
###############
resource "aws_api_gateway_method" "recipes_get" {
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  authorization = "NONE"
  http_method   = "GET"
  resource_id   = aws_api_gateway_resource.recipes.id
}

resource "aws_api_gateway_integration" "recipes_get" {
  http_method             = aws_api_gateway_method.recipes_get.http_method
  integration_http_method = "POST"
  resource_id             = aws_api_gateway_resource.recipes.id
  rest_api_id             = aws_api_gateway_rest_api.apigw.id
  type                    = "AWS_PROXY"
  uri                     = var.recipes_lambda_invoke_arn
}

################
##### POST #####
################
resource "aws_api_gateway_method" "recipes_post" {
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  authorization = "NONE"
  http_method   = "POST"
  resource_id   = aws_api_gateway_resource.recipes.id
}

resource "aws_api_gateway_integration" "recipes_post" {
  http_method             = aws_api_gateway_method.recipes_post.http_method
  integration_http_method = "POST"
  resource_id             = aws_api_gateway_resource.recipes.id
  rest_api_id             = aws_api_gateway_rest_api.apigw.id
  type                    = "AWS_PROXY"
  uri                     = var.recipes_lambda_invoke_arn
}
##################################
###### /recipes/{recipe_id} ######
##################################
resource "aws_api_gateway_resource" "recipe" {
  rest_api_id = aws_api_gateway_rest_api.apigw.id
  parent_id   = aws_api_gateway_resource.recipes.id
  path_part   = "{recipe_id}"
}

resource "aws_api_gateway_method" "recipe_get" {
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  authorization = "NONE"
  http_method   = "GET"
  resource_id   = aws_api_gateway_resource.recipe.id

  request_parameters = {
    "method.request.path.recipe_id" = true
  }
}

resource "aws_api_gateway_integration" "recipe_get" {
  http_method             = aws_api_gateway_method.recipe_get.http_method
  integration_http_method = "POST"
  resource_id             = aws_api_gateway_resource.recipe.id
  rest_api_id             = aws_api_gateway_rest_api.apigw.id
  type                    = "AWS_PROXY"
  uri                     = var.recipes_lambda_invoke_arn
}

resource "aws_api_gateway_method" "recipe_put" {
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  authorization = "NONE"
  http_method   = "PUT"
  resource_id   = aws_api_gateway_resource.recipe.id

  request_parameters = {
    "method.request.path.recipe_id" = true
  }
}

resource "aws_api_gateway_integration" "recipe_put" {
  http_method             = aws_api_gateway_method.recipe_put.http_method
  integration_http_method = "POST"
  resource_id             = aws_api_gateway_resource.recipe.id
  rest_api_id             = aws_api_gateway_rest_api.apigw.id
  type                    = "AWS_PROXY"
  uri                     = var.recipes_lambda_invoke_arn
}

resource "aws_api_gateway_method" "recipe_del" {
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  authorization = "NONE"
  http_method   = "DELETE"
  resource_id   = aws_api_gateway_resource.recipe.id

  request_parameters = {
    "method.request.path.recipe_id" = true
  }
}

resource "aws_api_gateway_integration" "recipe_del" {
  http_method             = aws_api_gateway_method.recipe_del.http_method
  integration_http_method = "POST"
  resource_id             = aws_api_gateway_resource.recipe.id
  rest_api_id             = aws_api_gateway_rest_api.apigw.id
  type                    = "AWS_PROXY"
  uri                     = var.recipes_lambda_invoke_arn
}

resource "aws_api_gateway_deployment" "apigw" {
  rest_api_id = aws_api_gateway_rest_api.apigw.id
  triggers = {
    redeployment = sha1(jsonencode([
      aws_api_gateway_resource.recipes.id,
      aws_api_gateway_method.recipes_get.id,
      aws_api_gateway_integration.recipes_get.id,
      aws_api_gateway_method.recipes_post.id,
      aws_api_gateway_integration.recipes_post.id,
      aws_api_gateway_resource.recipe.id,
      aws_api_gateway_method.recipe_get.id,
      aws_api_gateway_integration.recipe_get.id,
      aws_api_gateway_method.recipe_put.id,
      aws_api_gateway_integration.recipe_put.id,
      aws_api_gateway_method.recipe_del.id,
      aws_api_gateway_integration.recipe_del.id,
      aws_api_gateway_resource.upload.id,
      aws_api_gateway_integration.upload_post.id,
      aws_api_gateway_method.upload_post.id
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "apigw" {
  deployment_id = aws_api_gateway_deployment.apigw.id
  rest_api_id   = aws_api_gateway_rest_api.apigw.id
  stage_name    = "dev"
}
