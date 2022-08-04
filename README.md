# SecPar

A [Sec]rets Manager and [Par]ameter Store CLI tool that leverages the newly [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) to manage secrets.

[Secrets Manager vs Parameter Store](https://medium.com/awesome-cloud/aws-difference-between-secrets-manager-and-parameter-store-systems-manager-f02686604eae)

## Setup
One way to set credentials for AWS Rust SDK is `~/.aws/credentials`, one example as below:
```console
[default]
aws_access_key_id=<key_id>
aws_secret_access_key=<secret>
region=us-east-1
```
For alternative ways, please refer to the [SDK setup page](https://docs.aws.amazon.com/sdk-for-java/v1/developer-guide/setup-credentials.html)

## Usage Example

- List all the secret
```console
cargo run -- sec list
```
- Get specific secret value
```console
cargo run -- sec get --name <secret_name>
```
- Delete specific secret
```console
cargo run -- sec delete --name <secret_name>
```
- Describe specific secret
```console
cargo run -- sec describe --name <secret_name>
```
- Create specific secret
```console
cargo run -- sec create --name <secret_name> --secret <secret_value>
```
