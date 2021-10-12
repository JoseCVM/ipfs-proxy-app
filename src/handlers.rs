use super::models::{NewRequest, Request, NewUser, User, NewKey, Key};
use super::schema::requests::dsl::*;
use super::schema::users::dsl::*;
use super::schema::keys::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::client::Client;
use diesel::prelude::*;
use url::Url;
use diesel::dsl::{insert_into};
use std::vec::Vec;
use curl::easy::{Easy,List};
use std::io::Read;
use serde_json::Value;
use qstring::QString;

pub async fn resource_not_found(
) -> Result<HttpResponse, Error> {
   Ok(HttpResponse::NotFound().body("Resource does not exist"))
}

//TODO: Should return better error when user already exists
pub async fn new_user(
    db: web::Data<Pool>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let qs = QString::from(req.query_string());
    Ok(web::block(move || add_single_user(db, qs))
    .await
    .map(|user| HttpResponse::Created().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_user(
    db: web::Data<Pool>,
    qs: QString,
) -> Result<User, diesel::result::Error> {    
    let req_username = qs.get("username").unwrap();
    let conn = db.get().unwrap();
    let new_user = NewUser {
        username: req_username,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

pub async fn list_user(
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || list_all_users(db))
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn list_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}


pub async fn list_user_keys(
    db: web::Data<Pool>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let qs = QString::from(req.query_string());
    Ok(web::block(move || list_keys(db,qs))
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn list_keys(
    pool: web::Data<Pool>,
    qs: QString
) -> Result<Vec<Key>, diesel::result::Error>{ 
    let req_username = qs.get("username").unwrap();
    let conn = pool.get().unwrap();
    let user:User = users.filter(username.eq(req_username)).first(&conn)?;
    let items:Vec<Key> = keys.filter(userid.eq(user.id)).load::<Key>(&conn)?;
    Ok(items)
}

pub async fn generate_key(
    db: web::Data<Pool>,
    req: HttpRequest,
) ->  Result<HttpResponse, Error> {
    let qs = QString::from(req.query_string());
    Ok(web::block(move || insert_key_db(db,qs))
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn  insert_key_db( 
    db: web::Data<Pool>,
    qs: QString
)->Result<Key, diesel::result::Error>  {
    let conn = db.get().unwrap();
    let user:User = users.filter(username.eq(qs.get("username").unwrap())).first(&conn)?;
    
    let data = format!("{{\"client_id\":\"{}\",\"client_secret\":\"{}\",\"audience\":\"rust_ipfs_login\",\"grant_type\":\"client_credentials\"}}",std::env::var("KEY_ID").expect("KEY_ID must be set"),std::env::var("KEY_SECRET").expect("KEY_SECRET must be set"));
    
    let mut list = List::new();
    list.append("content-type: application/json").unwrap();

    let mut raw_data = data.as_bytes();
    let mut key_data = Vec::new();
    let mut handle = Easy::new();
    let base_url = std::env::var("KEY_PROVIDER").expect("KEY_PROVIDER must be set").clone();
    handle.url(base_url.as_str()).unwrap();
    handle.http_headers(list).unwrap();
    handle.post(true).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.read_function(|into| {
            Ok(raw_data.read(into).unwrap())
        }).unwrap();
        transfer.write_function(|new_data| {
            key_data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        
        transfer.perform().unwrap();
    }
    let v: Value = serde_json::from_str(String::from_utf8(key_data).expect("Found invalid UTF-8").as_str()).unwrap();
    let key = v["access_token"].to_string();
   
    let new_key = NewKey {
        api_key: &key,
        expires_in: 86400, // TODO: Default value, change later
        is_enabled: true,
        userid: user.id,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(keys).values(&new_key).get_result(&conn)?;
    Ok(res)
}

pub async fn disable_key(
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().body("Resource not implemented"))
}

pub async fn forward(
    db: web::Data<Pool>,
    req: HttpRequest,
    body: web::Bytes,
    url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let request_api_key = &req.headers().get("Authorization").unwrap().to_str().unwrap()[7..];

    log_request(db,req.clone(),body.clone());
    let mut new_url = url.get_ref().clone();

    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());
    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}


fn log_request(
    db: web::Data<Pool>,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<Request, diesel::result::Error> {
    let request_api_key = &req.headers().get("Authorization").unwrap().to_str().unwrap()[7..];
    let request_api_call = req.uri().path();
    let req_size:i32 = body.len() as i32;
    println!("{}{}{}",request_api_key,request_api_call,req_size);
    let conn = db.get().unwrap();
    let new_request = NewRequest {
        api_key: &request_api_key,
        api_call: &request_api_call,
        request_size: req_size,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(requests).values(&new_request).get_result(&conn)?;
    Ok(res)
}


