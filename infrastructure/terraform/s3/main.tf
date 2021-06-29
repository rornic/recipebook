
resource "aws_s3_bucket" "recipe_images" {
  bucket = "${var.name}-images"
  acl    = "public-read"

  tags = {
    Name = "recipe_images"
  }

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["PUT", "POST"]
    allowed_origins = ["*"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }

}
