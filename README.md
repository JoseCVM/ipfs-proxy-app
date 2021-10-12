### Project Description

Reverse proxy that sits between a local ipfs node and the internet. You can control who can access the ipfs api through this proxy, and collect telemetry data on calls made (Who made it, request size, etc)


### Usage
You need to have cargo, diesel and postgresql installed and working. You will need to have a database at DATABASE_URL with permissions for the user running the server. Open and close ports at will
Apps used:
ipfs_api: Redirects all requests made here (port 8080) to the same url in the local ipfs node. Effectively routes from localhost:8080 to localhost:5001
This port listens to whatever endpoints your local ipfs node does.
keygen_api: API for managing tenants, listens on port 9090 - Giving out keys, collecting telemetry data, disabling tenants etc. Currently NO certification, so ideally you should either authenticate calls made here with a token, or just use it internally (Careful!)
Current keygen_api endpoints:
"/user/new?username=NAME" - Creates a new tenant with a unique username
"/user/list" - Lists all users
"/user/keys?username=NAME" - Lists all keys given to NAME
"/key/generate?username=NAME" - Gives out a new key to NAME

### Further development

This can be used as the basis for a tenant management model through which you can control your ipfs api. Current priority would be in refactoring the code to facilitate future work. Later, finish opening up the API endpoints for key management and telemetry.
