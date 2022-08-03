# SecPar

A [Sec]rets Manager and [Par]ameter Store CLI tool that leverages the newly [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) to manage secrets.

[Secrets Manager vs Parameter Store](https://medium.com/awesome-cloud/aws-difference-between-secrets-manager-and-parameter-store-systems-manager-f02686604eae)

## Setup
AWS Rust SDK assumes that you have a valid `default` profile defined in `~/.aws/config`, one example as below:
```console
[default]
aws_access_key_id=<key_id>
aws_secret_access_key=<secret>
region=us-east-1
```

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
