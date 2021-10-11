### Project Description

This is a toy project that implements a simple authenticated proxy server for IPFS. 
The idea is that you can block incoming traffic from the ports used by the IPFS node you run locally, and manually add handlers for each ipfs api call you want to expose

The authentication is done externally by Auth0. Afterwards, you can give out Auth0 api keys to users and let them use the ipfs calls you exposed. For example, you could use this
as a service on your application and give out keys to users as they register.

### What we have so far

Authentication, enabling/disabling of authentication keys, only authenticated users can run API calls. Currently keys are supplied on-demand by the administrator.

All requests are logged to the database (postgresql, using diesel to connect from within rust) as a pair of key/request, so we can retrieve all requests. This isn't exposed as an endpoint to this API for obvious reasons - we don't want outside users looking at our telemetry data. Instead, you should just check it locally.

It is by design that we don't just forward all API calls and instead route them one by one - we only expose what we need/want to expose.

### To Do:

All API calls done to our server need to replace / for _ (swarm/peers becomes swarm_peers). Fix this so OUR_SERVER/api/v0/swarm/peers works.

Currently requests are mocked by a simple CURL that prints output to stdout - make a more robust reqwest client class for this application so the addition/removal of new endpoints is simple, just take the existing request, strip the API key, edit URI, forward to local ipfs node and return the response.

Improve on logging, actix has great logger middleware that could be used here.
