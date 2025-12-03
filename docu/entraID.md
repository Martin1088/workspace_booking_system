# Microsoft Entra ID (Azure AD) Login – Setup Guide

This application supports OAuth2 / OpenID Connect login via Microsoft Entra ID.
Follow these steps to configure the integration.



1. Create an App Registration
    1.	Go to Azure Portal → Microsoft Entra ID → App registrations
    2.	Click New registration
    3.	Name it: WorkspaceApp (or any name)
    4.	Supported account types:
          •	Single-tenant (recommended — only your org can sign in)
          or
          •	Multi-tenant (allows any Microsoft account to sign in)
    5.	Click Register

2. Configure Redirect URI
   - In your app registration:
   - Authentication → Add a Redirect URI
   
   Example for Azure Container Apps:
```
https://<yourapp>.westeurope.azurecontainerapps.io/api/oauth2/authentik/callback/cookie
```
- Type = Web
- Save the configuration

3. Generate a Client Secret
    1.	Go to Certificates & secrets
    2.	Click New client secret
    3.	Copy the Secret Value (not the ID!)
    4.	Store it securely

4. Required API Permissions

Under API permissions:

Required delegated permissions:
•	openid
•	profile
•	email

These allow the app to read the user profile.