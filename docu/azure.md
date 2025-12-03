# Azure Cloud via Shell
1. You need an Azure Account [AzureMicrosoft](https://azure.microsoft.com/de-de/pricing/purchase-options/azure-account?icid=get-started)
    and register for EntraID [Instruction](entraID.md)
2. Install [Azure Shell](https://learn.microsoft.com/de-de/cli/azure/install-azure-cli?view=azure-cli-latest)
3. Login to your Account.
```shell
az login
```
Those two commands registered Azure resource providers in your subscription.
They are required creating Container Apps and Log Analytics workspaces, if your subscription didn’t have those providers enabled yet.
```
az provider register -n Microsoft.App --wait
az provider register -n Microsoft.OperationalInsights --wait
```
4. Create a group for yoru workspace in your region
```
az group create -n workspace-rg -l westeurope
```
5. 
```
az containerapp env create \
  --name workspace-env \
  --resource-group workspace-rg \
  --location westeurope
```
6.
```
az containerapp create \
  --name workspace-booking \
  --resource-group workspace-rg \
  --environment workspace-env \
  --image ghcr.io/martin1088/workspace-booking-system:sha-bd380a3 \
  --target-port 5150 \
  --ingress external \
  --env-vars DATABASE_URL="mysql://worker:worker@workspace-db.mysql.database.azure.com/mrbs?ssl-mode=REQUIRED"
```
The command registered Azure MySQL in your subscription.
```
az provider register -n Microsoft.DBforMySQL --wait
```
7. Create MySQL database as Azure service.
```
az mysql flexible-server create \
  -g workspace-rg \
  -n workspace-db \
  -l westeurope \
  --tier Burstable \
  --sku-name Standard_B1ms \
  --version 8.0 \
  --admin-user worker \
  --admin-password "worker"
```
8. Open the firewall to update database and fill in the data.
```
az mysql flexible-server firewall-rule create \
  --resource-group workspace-rg \
  --name workspace-db \
  --rule-name AllowAllAzureIPs \
  --start-ip-address 0.0.0.0 \
  --end-ip-address 0.0.0.0
```
Find the domain Azure is hosting the database.
```
  HOST=$(az mysql flexible-server show -g workspace-rg -n workspace-db --query fullyQualifiedDomainName -o tsv)
```
```
docker run --rm -e MYSQL_PWD=worker mysql:8.0 \
  mysql -h "$HOST" -u worker@workspace-db --ssl-mode=REQUIRED \
  -e "create database mrbs;"
```
```
  docker run --rm -i -e MYSQL_PWD=worker mysql:8.0 \
    mysql -h "$HOST" -u worker --ssl-mode=REQUIRED \
    < backup.sql
```

9. Create container for [Workspace-booking-system](../README.md) 
```
az containerapp create \
  --name workspace-booking \
  --resource-group workspace-rg \
  --environment workspace-env \
  --image ghcr.io/martin1088/workspace-booking-system:sha-bd380a3 \
  --ingress external --target-port 5150 \
  --cpu 0.5 --memory 1.0Gi \
  --env-vars DATABASE_URL="mysql://worker:worker@workspace-db.mysql.database.azure.com/mrbs?ssl-mode=REQUIRED"
```
10. Store all variables and the oauth client secret in the container from EntraID.
```
az containerapp secret set \
  --name workspace-booking \
  --resource-group workspace-rg \
  --secrets oauth-client-secret="<secret>"
```
```
az containerapp update \
  --name workspace-booking \
  --resource-group workspace-rg \
  --set-env-vars \
    OAUTH_CLIENT_ID="<cleint-id>" \
    OAUTH_CLIENT_SECRET=secretref:oauth-client-secret \
    AUTH_URL="https://login.microsoftonline.com/<tenant-id>/oauth2/v2.0/authorize" \
    TOKEN_URL="https://login.microsoftonline.com/<tenant-id>/oauth2/v2.0/token" \
    PROFILE_URL="https://graph.microsoft.com/oidc/userinfo" \
    OAUTH_SCOPES="openid profile email" \
    REDIRECT_URL="https://<workspace-booking.<>.westeurope.azurecontainerapps.io>/api/oauth2/authentik/callback/cookie" \
    PROTECTED_URL="https://<workspace-booking.<>.westeurope.azurecontainerapps.io>/"
    PROTECTED_URL="https://<workspace-booking.<>.westeurope.azurecontainerapps.io>/" \
    REDIRECT_URL="https://<workspace-booking.<>.westeurope.azurecontainerapps.io>/api/oauth2/authentik/callback/cookie"
```
