resource "aws_dynamodb_table" "recipes" {
  name           = "${var.name}-recipes"
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "uuid"


  attribute {
    name = "uuid"
    type = "S"
  }

  tags = {
    Name        = "${var.name}-recipes"
    Environment = "dev"
    Project     = "recipebook"
  }
}
