#[macro_use]
extern crate diesel;


use actix_web::{dev::ServiceRequest, web, App, Error,  HttpServer};
use actix_web::client::Client;
use diesel::prelude::*;
use url::Url;
use std::net::{ToSocketAddrs, SocketAddr};
use futures::future;
use diesel::r2d2::{self, ConnectionManager};
mod auth;
mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // TODO: Make a friendier CLI, help, etc    

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let forward_url = Url::parse(&format!(
        "http://{}",
        SocketAddr::from(([127, 0, 0, 1], 5001))
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    )).unwrap();

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    // TODO: clean this up so you don't repeat the new App code, but also don't break all the closures
    // TODO: Allow user to specify which calls he wants to block via some config file    
   
    let p1 = pool.clone();
    let p2 = pool.clone();
    /*======================================================================*/
    let ipfs_api = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
        .data(Client::new())
        .data(forward_url.clone())
        .data(p1.clone())       
        .wrap(auth)
        .default_service(web::route().to(handlers::forward))
    }).bind("127.0.0.1:8080")?.run();

    let keygen_api =  HttpServer::new(move || {
        App::new()
        .data(p2.clone())  
        .route(
            "/user/new",
            web::post().to(handlers::new_user),
        )
        .route(
            "/user/list",
            web::post().to(handlers::list_user),
        )
        .route(
            "/user/keys",
            web::post().to(handlers::list_user_keys),
        )
        .route(
            "/key/generate",
            web::post().to(handlers::generate_key),
        )
        .route(
            "/key/disable",
            web::post().to(handlers::disable_key),
        )
        .default_service(web::route().to(handlers::resource_not_found))
    }).bind("127.0.0.1:9090")?.run(); 
    
    future::try_join(ipfs_api, keygen_api).await;
    Ok(())
}
