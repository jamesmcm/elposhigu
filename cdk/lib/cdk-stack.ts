import * as cdk from "@aws-cdk/core";
import * as ec2 from "@aws-cdk/aws-ec2";
import * as ecs from "@aws-cdk/aws-ecs";
import * as iam from "@aws-cdk/aws-iam";
import * as ecs_patterns from "@aws-cdk/aws-ecs-patterns";
import { DockerImageAsset } from "@aws-cdk/aws-ecr-assets";
// import * as ecr from "@aws-cdk/aws-ecr";

var path = require("path");

export class CdkStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    const asset = new DockerImageAsset(this, "MyBuildImage", {
      directory: path.join(__dirname, "/../../backend/"),
    });

    // const repo = new ecr.Repository(this, "MyRepo", {});

    const vpc = new ec2.Vpc(this, "MyVpc", {
      maxAzs: 2, // Default is all AZs in region
    });
    const role = new iam.Role(this, "MyRole", {
      assumedBy: new iam.AccountPrincipal(this.account),
      roleName: "elposhigu_role",
    });

    const cluster = new ecs.Cluster(this, "MyCluster", {
      vpc: vpc,
    });
    // TODO: Try without load balancer
    // Create a load-balanced Fargate service and make it public
    const fargate = new ecs_patterns.ApplicationLoadBalancedFargateService(
      this,
      "MyFargateService",
      {
        cluster: cluster,
        cpu: 256,
        desiredCount: 1,
        taskImageOptions: {
          image: ecs.ContainerImage.fromDockerImageAsset(asset),
          containerPort: 8080,
          taskRole: role,
          // Could use secrets
          // secrets: {},
          environment: {},
        },
        memoryLimitMiB: 512,
        assignPublicIp: true,
      }
    );

    console.log(fargate.cluster.vpc.publicSubnets);
  }
}
