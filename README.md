# SecPar

A [Sec]rets Manager and [Par]ameter Store CLI tool built on the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust).

When `--name` is omitted on interactive commands, secpar fetches the live resource list and presents a selection menu so you can pick without leaving the terminal.

[Secrets Manager vs Parameter Store](https://medium.com/awesome-cloud/aws-difference-between-secrets-manager-and-parameter-store-systems-manager-f02686604eae)

## Setup

Credentials are resolved in this order:

1. `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` environment variables
2. `~/.aws/credentials`
3. `~/.aws/config`
4. IAM instance / task / IRSA roles (EC2, ECS, EKS)

Example `~/.aws/credentials`:
```ini
[default]
aws_access_key_id=<key_id>
aws_secret_access_key=<secret>
region=us-east-1
```

For more options see the [AWS SDK credential setup guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html).

## Global Options

Available on all commands:

| Flag | Env var | Default |
|------|---------|---------|
| `--region <REGION>` | `AWS_REGION` | `us-east-1` |
| `--profile <PROFILE>` | `AWS_PROFILE` | *(none)* |

```console
secpar --region eu-west-1 --profile staging sec list
```

## Usage

### Secrets Manager

- List all secrets (formatted table)
```console
secpar sec list
```

- Get a secret value — pass `--name` or omit it for an interactive menu
```console
secpar sec get --name <secret_name>
secpar sec get
```

- Describe a secret — pass `--name` or omit it for an interactive menu
```console
secpar sec describe --name <secret_name>
secpar sec describe
```

- Create a secret
```console
secpar sec create --name <secret_name> --secret <secret_value>
```

- Delete a secret — pass `--name` or omit it for an interactive menu; always asks for confirmation
```console
secpar sec delete --name <secret_name>
secpar sec delete
```

### Parameter Store

- List all parameters (formatted table)
```console
secpar par list
```

- Get a parameter value — pass `--name` or omit it for an interactive menu
```console
secpar par get --name <parameter_name>
secpar par get
```

- Create a parameter (stored as `SecureString`)
```console
secpar par create --name <parameter_name> --value <parameter_value>
```

- Delete a parameter — pass `--name` or omit it for an interactive menu; always asks for confirmation
```console
secpar par delete --name <parameter_name>
secpar par delete
```

- Apply a bulk parameter spec file
```console
secpar par apply --path <path_to_spec_file>
```

### Parameter Store Spec Format

Used by `par apply`. YAML format; each entry is `name:value` separated by a colon:

```yaml
parameters:
  - /secpar/TEST:TEST_VALUE
  - /secpar/qa/SASL_USERNAME:USERNAME
```
