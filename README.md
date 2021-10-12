# Project Description

Reverse proxy that sits between a local ipfs node and the internet. You can control who can access the ipfs api through this proxy, and collect telemetry data on calls made (Who made it, request size, etc). This projects uses auth0 as a higher authority to issue the tokens used by tenants to accesss the API

# Installation
Clone this repo and navigate to its folder

Check that you have postgresql installed: https://www.postgresql.org/download/linux/ubuntu/ and that your current user has adequate privileges

Check that you have cargo installed: ```curl https://sh.rustup.rs -sSf | sh``` or https://doc.rust-lang.org/cargo/getting-started/installation.html

Check that you have diesel installed: ```cargo install diesel_cli --no-default-features --features postgres```

Fill the KEY_ID and KEY_SECRET (By asking me for them or by securing a new key provider at auth0 or any other authority, in which case you'll also need to change KEY_PROVIDER and AUTHORITY)

run:

```
diesel setup
diesel migration run
cargo run
```

First compilation might take a while, mind you

# Usage
You need to have cargo, diesel and postgresql installed and working. You will need to have a database at DATABASE_URL with permissions for the user running the server. Open and close ports at will to achieve desired result (Better management of this is a feature to come). Once the server is running, you can test by using ```curl``` on another terminal like so:

```
curl -X POST -v 127.0.0.1:9090/user/new?username=john
curl -X POST -v 127.0.0.1:9090/key/generate?username=john

``` 
Take note of the output from this command - its Johns new key that he can use to access the ipfs api. 

The next example assumes you have a key in the $TOKEN env variable
```
curl -H "Authorization: Bearer $TOKEN" -X POST -F file=@myfile "http://127.0.0.1:8080/api/v0/add
``` 

On the example above, we created a tenant John, gave him a new temporary token, and then he made an "add" call to the ipfs api through our proxy server using his new key.


### ipfs_api
  - Redirects all requests made here (port 8080) to the same url in the local ipfs node. Effectively routes from localhost:8080 to localhost:5001
This port listens to whatever endpoints your local ipfs node does.

### keygen_api
  -API for managing tenants, listens on port 9090 - Giving out keys, collecting telemetry data, disabling tenants etc. Currently NO certification, so ideally you should either authenticate calls made here with a token, or just use it internally (Careful!)

## Current keygen_api endpoints:

"/user/new?username=NAME" - Creates a new tenant with a unique username

"/user/list" - Lists all users

"/user/keys?username=NAME" - Lists all keys given to NAME

"/key/generate?username=NAME" - Gives out a new key to NAME

# Further development

This can be used as the basis for a tenant management model through which you can control your ipfs api. Current priority would be in refactoring the code to facilitate future work. Later, finish opening up the API endpoints for key management and telemetry. Create a front-end app that can use this + tenant authentication to expose a key-issuing/telemetry dashboard
