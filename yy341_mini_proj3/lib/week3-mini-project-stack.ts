import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as iam from 'aws-cdk-lib/aws-iam';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class Week3MiniProjectStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // make an S3 bucket that enable versioning and encryption
    const bucket = new cdk.aws_s3.Bucket(this, 'week3-mini-project', {
      versioned: true,
      encryption: cdk.aws_s3.BucketEncryption.S3_MANAGED,
      removalPolicy: cdk.RemovalPolicy.DESTROY
    });
    
    bucket.grantReadWrite(new iam.AccountRootPrincipal());
  }
}
