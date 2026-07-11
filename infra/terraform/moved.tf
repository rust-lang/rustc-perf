# The stack was originally applied from a config revision that declared these
# resources behind `count` conditionals, so the state stores them with an [0]
# index. The current config declares them unconditionally; these moves adopt
# the existing resources instead of destroying and recreating them.

moved {
  from = aws_subnet.public[0]
  to   = aws_subnet.public
}

moved {
  from = aws_route_table_association.public[0]
  to   = aws_route_table_association.public
}

moved {
  from = aws_ecr_repository.site[0]
  to   = aws_ecr_repository.site
}

moved {
  from = aws_ecr_lifecycle_policy.site[0]
  to   = aws_ecr_lifecycle_policy.site
}

moved {
  from = aws_iam_role.instance[0]
  to   = aws_iam_role.instance
}

moved {
  from = aws_iam_role_policy.app[0]
  to   = aws_iam_role_policy.app
}

moved {
  from = aws_iam_role_policy_attachment.ssm_core[0]
  to   = aws_iam_role_policy_attachment.ssm_core
}

moved {
  from = aws_iam_instance_profile.instance[0]
  to   = aws_iam_instance_profile.instance
}

moved {
  from = aws_s3_bucket.backups[0]
  to   = aws_s3_bucket.backups
}

moved {
  from = aws_s3_bucket_lifecycle_configuration.backups[0]
  to   = aws_s3_bucket_lifecycle_configuration.backups
}

moved {
  from = aws_s3_bucket_public_access_block.backups[0]
  to   = aws_s3_bucket_public_access_block.backups
}

moved {
  from = aws_s3_bucket_server_side_encryption_configuration.backups[0]
  to   = aws_s3_bucket_server_side_encryption_configuration.backups
}

moved {
  from = aws_s3_bucket_policy.backups[0]
  to   = aws_s3_bucket_policy.backups
}
