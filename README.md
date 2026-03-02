# SecPar

[![crates.io](https://img.shields.io/crates/v/secpar.svg)](https://crates.io/crates/secpar)
[![docs.rs](https://docs.rs/secpar/badge.svg)](https://docs.rs/secpar/latest/secpar/)
[![CI](https://github.com/riyaolin/secpar/actions/workflows/ci.yml/badge.svg)](https://github.com/riyaolin/secpar/actions/workflows/ci.yml)

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
Asks for confirmation before creating. Pass `-y` / `--yes` to skip.
```console
$ secpar sec create --name staging/token --secret 'tok-xyz789'
? Create secret 'staging/token'? (y/N) › y
✅ Secret 'staging/token' created.
ℹ️  ARN: arn:aws:secretsmanager:us-east-1:000000000000:secret:staging/token-PSWZgn
```

#### `sec delete`
Pass `--name` or omit it for an interactive selection menu. Always asks for confirmation. Pass `-y` / `--yes` to skip the prompt.
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

#### `sec apply`
Bulk-create secrets from a YAML spec file. Supports compact and expanded entry formats — see [Secrets Manager Spec Format](#secrets-manager-spec-format). Secrets that already exist are skipped. Asks for confirmation before applying; pass `-y` / `--yes` to skip.
```console
$ secpar sec apply --path ./templates/secrets_template.yaml
? Apply secrets from 'templates/secrets_template.yaml'? (y/N) › y
ℹ️  [1/3] created 'prod/api/key' → arn:aws:secretsmanager:us-east-1:000000000000:secret:prod/api/key-AbCdEf
ℹ️  [2/3] created 'prod/db/password' → arn:aws:secretsmanager:us-east-1:000000000000:secret:prod/db/password-GhIjKl
ℹ️  [3/3] created 'prod/oauth/client-secret' → arn:aws:secretsmanager:us-east-1:000000000000:secret:prod/oauth/client-secret-MnOpQr
✅ Secrets applied successfully.
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
Stored as `SecureString`. Asks for confirmation before creating. Pass `-y` / `--yes` to skip.
```console
$ secpar par create --name /staging/feature-flag --value true
? Create parameter '/staging/feature-flag'? (y/N) › y
✅ Parameter '/staging/feature-flag' created.
```

#### `par delete`
Pass `--name` or omit it for an interactive selection menu. Always asks for confirmation. Pass `-y` / `--yes` to skip the prompt.
```console
$ secpar par delete --name /staging/feature-flag
? Delete '/staging/feature-flag'? (y/N) › y
✅ Parameter '/staging/feature-flag' deleted.
```

#### `par apply`
Bulk-load parameters from a YAML spec file. Supports compact and expanded entry formats — see [Parameter Store Spec Format](#parameter-store-spec-format). Asks for confirmation before applying; pass `-y` / `--yes` to skip.
```console
$ secpar par apply --path ./templates/parameter_store_template.yaml
? Apply parameters from 'templates/parameter_store_template.yaml'? (y/N) › y
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
just local sec apply --path ./templates/secrets_template.yaml

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

Used by `par apply`. Two entry formats are supported and can be mixed freely in the same file.

**Compact** — `name:value` inline, split on the first colon:

```yaml
parameters:
  - /prod/db/host:db.internal.example.com
  - /prod/db/port:5432
```

**Expanded** — explicit `name` and `value` keys. Required when the value itself contains colons (URLs, connection strings, etc.):

```yaml
parameters:
  - name: /prod/db/url
    value: postgres://user:pass@db.internal:5432/mydb
  - name: /prod/api/key
    value: sk-abc123
```

Both forms can be mixed in the same file:

```yaml
parameters:
  # compact
  - /prod/db/port:5432
  # expanded (value contains colons)
  - name: /prod/db/url
    value: postgres://user:pass@db.internal:5432/mydb
```

A malformed compact entry (missing colon) is caught at parse time and the entire `apply` is aborted before any AWS calls are made.

### Secrets Manager Spec Format

Used by `sec apply`. Two entry formats are supported and can be mixed freely in the same file.

**Compact** — `name:secret` inline, split on the first colon:

```yaml
secrets:
  - prod/api/key:sk-abc123xyz
```

**Expanded** — explicit `name` and `secret` keys. Required when the secret value contains colons (JSON, URLs, etc.):

```yaml
secrets:
  - name: prod/db/password
    secret: '{"user":"admin","pass":"s3cr3t"}'
  - name: prod/oauth/client-secret
    secret: oauth2://client_id:client_secret@auth.example.com
```

Both forms can be mixed in the same file:

```yaml
secrets:
  # compact
  - prod/api/key:sk-abc123xyz
  # expanded (secret value contains colons)
  - name: prod/db/password
    secret: '{"user":"admin","pass":"s3cr3t"}'
```

A malformed compact entry (missing colon) is caught at parse time and the entire `apply` is aborted before any AWS calls are made. Secrets that already exist in Secrets Manager are skipped rather than aborting.
