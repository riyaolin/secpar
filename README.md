# SecPar

A [Sec]rets Manager and [Par]ameter Store CLI tool built on the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust).

When `--name` is omitted on interactive commands, secpar fetches the live resource list and presents a selection menu so you can pick without leaving the terminal.

[Secrets Manager vs Parameter Store](https://medium.com/awesome-cloud/aws-difference-between-secrets-manager-and-parameter-store-systems-manager-f02686604eae)

## Installation

**Prerequisites:** [Rust toolchain](https://rustup.rs) (1.91+).

### From crates.io

```console
cargo install secpar
```

### From source

```console
git clone https://github.com/riyaolin/secpar
cd secpar
cargo install --path .
```

Verify the install:

```console
secpar --version
```

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

#### `sec list`
```console
$ secpar sec list
┌──────────────────┬──────────────────────────────────────────────────────────────────┬─────────────────────────────┐
│ NAME             ┆ ARN                                                              ┆ LAST CHANGED                │
╞══════════════════╪══════════════════════════════════════════════════════════════════╪═════════════════════════════╡
│ prod/db/password ┆ arn:aws:secretsmanager:us-east-1:000000000000:secret:prod/db/... ┆ 2026-03-02T01:27:23.089076Z │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ prod/api/key     ┆ arn:aws:secretsmanager:us-east-1:000000000000:secret:prod/api/... ┆ 2026-03-02T01:27:23.204993Z │
└──────────────────┴──────────────────────────────────────────────────────────────────┴─────────────────────────────┘
ℹ️  2 secret(s) found.
```

#### `sec get`
Pass `--name` or omit it for an interactive selection menu.
```console
$ secpar sec get --name prod/api/key
🔑 prod/api/key
sk-abc123xyz
```

#### `sec describe`
Pass `--name` or omit it for an interactive selection menu.
```console
$ secpar sec describe --name prod/db/password
ℹ️  Secret details
  Name          : prod/db/password
  ARN           : arn:aws:secretsmanager:us-east-1:000000000000:secret:prod/db/password-VrSGdO
  Description   : -
  Last Changed  : 2026-03-02T01:27:23.089076Z
  Last Accessed : -
  Rotation      : disabled
```

#### `sec create`
```console
$ secpar sec create --name staging/token --secret 'tok-xyz789'
✅ Secret 'staging/token' created.
ℹ️  ARN: arn:aws:secretsmanager:us-east-1:000000000000:secret:staging/token-PSWZgn
```

#### `sec delete`
Pass `--name` or omit it for an interactive selection menu. Always asks for confirmation.
```console
$ secpar sec delete --name staging/token
? Delete 'staging/token'? (y/N) › y
✅ Secret 'staging/token' deleted.
```

Pass `--force` to bypass the 7-day recovery window and skip the confirmation prompt.
```console
$ secpar sec delete --name staging/token --force
✅ Secret 'staging/token' deleted (force).
```

---

### Parameter Store

#### `par list`
```console
$ secpar par list
┌─────────────────┬──────────────┬─────────────────────────────┐
│ NAME            ┆ TYPE         ┆ LAST MODIFIED               │
╞═════════════════╪══════════════╪═════════════════════════════╡
│ /prod/db/host   ┆ SecureString ┆ 2026-03-02T01:27:23.737999Z │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ /prod/db/port   ┆ SecureString ┆ 2026-03-02T01:27:23.881999Z │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ /prod/cache/url ┆ SecureString ┆ 2026-03-02T01:27:23.996Z    │
└─────────────────┴──────────────┴─────────────────────────────┘
ℹ️  3 parameter(s) found.
```

#### `par get`
Pass `--name` or omit it for an interactive selection menu.
```console
$ secpar par get --name /prod/db/host
🔑 /prod/db/host
db.internal.example.com
```

#### `par create`
Stored as `SecureString`.
```console
$ secpar par create --name /staging/feature-flag --value true
✅ Parameter '/staging/feature-flag' created.
```

#### `par delete`
Pass `--name` or omit it for an interactive selection menu. Always asks for confirmation.
```console
$ secpar par delete --name /staging/feature-flag
? Delete '/staging/feature-flag'? (y/N) › y
✅ Parameter '/staging/feature-flag' deleted.
```

#### `par apply`
Bulk-load parameters from a YAML spec file.
```console
$ secpar par apply --path ./templates/parameter_store_template.yaml
📂 Applying parameters from 'templates/parameter_store_template.yaml'…
✅ Parameters applied successfully.
```

## Local Testing with LocalStack

[LocalStack](https://localstack.io) runs Secrets Manager and Parameter Store locally in Docker so you can try secpar without touching real AWS resources.

**Prerequisites:** [Docker](https://docs.docker.com/get-docker/) and [just](https://github.com/casey/just#installation).

### Start / stop

```console
just localstack-up    # start LocalStack in the background (waits until ready)
just localstack-down  # stop and remove the volume
```

### Run commands locally

Use `just local` as a drop-in for `secpar`. It sets the dummy credentials and endpoint automatically:

```console
# Secrets Manager
just local sec create --name my-secret --secret '{"key":"value"}'
just local sec list
just local sec get --name my-secret
just local sec describe --name my-secret
just local sec delete --name my-secret

# Parameter Store
just local par create --name /my/param --value s3cr3t
just local par list
just local par get --name /my/param
just local par apply --path ./templates/parameter_store_template.yaml
just local par delete --name /my/param
```

Or set the variables yourself and run the binary directly:

```console
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_ENDPOINT_URL=http://localhost:4566
secpar --region us-east-1 sec list
```

### Parameter Store Spec Format

Used by `par apply`. YAML format; each entry is `name:value` separated by a colon:

```yaml
parameters:
  - /secpar/TEST:TEST_VALUE
  - /secpar/qa/SASL_USERNAME:USERNAME
```
