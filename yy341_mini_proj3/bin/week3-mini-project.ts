#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { Week3MiniProjectStack } from '../lib/week3-mini-project-stack';
import { DefaultStackSynthesizer, Stack } from 'aws-cdk-lib';

const app = new cdk.App();
new Week3MiniProjectStack(app, 'Week3MiniProjectStack', {
  // add necessary variables to create the S3 bucket and deploy it 
  
  /* If you don't specify 'env', this stack will be environment-agnostic.
   * Account/Region-dependent features and context lookups will not work,
   * but a single synthesized template can be deployed anywhere. */

  /* Uncomment the next line to specialize this stack for the AWS Account
   * and Region that are implied by the current CLI configuration. */
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },

  /* Uncomment the next line if you know exactly what Account and Region you
   * want to deploy the stack to. */
  // env: { account: '123456789012', region: 'us-east-1' },

  /* For more information, see https://docs.aws.amazon.com/cdk/latest/guide/environments.html */
  synthesizer: new cdk.BootstraplessSynthesizer({})
  
});