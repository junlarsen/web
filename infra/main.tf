resource "aws_s3_bucket" "codes" {
  bucket   = "jun.codes"
  acl      = "public-read"
  provider = aws.london

  policy = <<EOF
    {
      "Version":"2008-10-17",
      "Statement":[{
        "Sid":"AllowPublicRead",
        "Effect":"Allow",
        "Principal": {"AWS": "*"},
        "Action":["s3:GetObject"],
        "Resource":["arn:aws:s3:::jun.codes/*"]
      }]
    }
  EOF

  website {
    error_document = "index.html"
    index_document = "index.html"
  }
}

resource "aws_route53_record" "jun_codes" {
  zone_id = data.aws_route53_zone.domain.zone_id
  name    = data.aws_route53_zone.domain.name
  type    = "A"
  alias {
    name                   = aws_cloudfront_distribution.cdn.domain_name
    zone_id                = aws_cloudfront_distribution.cdn.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_cloudfront_distribution" "cdn" {
  origin {
    origin_id   = "jun.codes"
    domain_name = "jun.codes.s3.amazonaws.com"
  }

  aliases             = ["jun.codes"]
  enabled             = true
  default_root_object = "index.html"

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "jun.codes"

    forwarded_values {
      query_string = true
      cookies {
        forward = "none"
      }
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  price_class = "PriceClass_100"

  restrictions {
    geo_restriction {
      restriction_type = "none"
      locations        = []
    }
  }

  viewer_certificate {
    acm_certificate_arn = aws_acm_certificate.jun_codes.arn
    ssl_support_method  = "vip"
  }
}

resource "aws_acm_certificate" "jun_codes" {
  domain_name       = "jun.codes"
  validation_method = "DNS"
  provider          = aws.virginia

  lifecycle {
    create_before_destroy = true
  }
}
