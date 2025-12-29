# Kubernetes
This connects **concept → implementation**.
For local testing and deployment.
(at the moment solution)

## Installation

1. Create required secrets
2. Install the Helm chart
3. Access the application
## Required Kubernetes Secrets

Before installing the Helm chart, the following secrets must exist
in the target namespace.

## Secrets handling (important)

All sensitive values (OAuth, database credentials) must be provided
as existing Kubernetes Secrets and are referenced via `values.yaml`.

This is intentional and follows Kubernetes & Helm best practices.
1. create oauth secrets
```shell
kubectl create secret generic oauth-env --from-env-file=.env
```
### OAuth secret (`oauth-env`)

The application expects the following environment variables:

- `OAUTH_CLIENT_ID`
- `OAUTH_CLIENT_SECRET`
- `AUTH_URL`
- `TOKEN_URL`
- `REDIRECT_URL`
- `PROFILE_URL`
- `PROTECTED_URL`
- `OAUTH_SCOPES`

## Setup mysql
insert tables and database manual
```shell
kubectl exec -i mysql-0 -- mysql -uworker -pworker mrbs < backup.sql
```
check if database and tables are present
```shell
kubectl exec -it mysql-0 -- mysql -uworker -pworker -e "SHOW DATABASES;"
kubectl exec -it mysql-0 -- mysql -uworker -pworker mrbs -e "SHOW TABLES;"
```
(dev) test on localhost:5150
```shell
kubectl port-forward svc/workspace-booking-app 5150:5150
```

```shell
kubectl create secret generic oauth-env --from-env-file=.env
```