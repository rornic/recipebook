output "apigw_arn" {
  value = "${aws_api_gateway_rest_api.apigw.execution_arn}/*/*/*"
}
