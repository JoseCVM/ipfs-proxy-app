use super::models::{NewRequest, Request};
use super::schema::requests::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use diesel::dsl::{insert_into};
use std::vec::Vec;
use std::io::{stdout, Write};
use curl::easy::Easy;

pub async fn swarm_peers(
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut buf = Vec::new();
    let mut easy = Easy::new();
    easy.url("http://127.0.0.1:5001/api/v0/swarm/peers").unwrap(); 
    easy.post(true).unwrap();
    easy.write_function(move |data| {
        buf.extend_from_slice(data);
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    return Ok(HttpResponse::Ok().body("API call went through"));
}

pub async fn route_calls(
    db: web::Data<Pool>,
    param: web::Path<String>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let request_key = get_api_key(&_req).unwrap().to_string();
    println!("{:?} {:?}",request_key, param.to_string());
    let response = match param.as_str() {
        "swarm_peers" => swarm_peers(_req),
        _ => {return Ok(HttpResponse::NotFound().body("Resource does not exist"));},
    };
    log_request(db,request_key, param.to_string());
    return response.await;
}

fn get_api_key<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}

fn log_request(
    db: web::Data<Pool>,
    request_api_key: String,
    request_api_call: String
) -> Result<Request, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_request = NewRequest {
        api_key: &request_api_key,
        api_call: &request_api_call,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(requests).values(&new_request).get_result(&conn)?;
    Ok(res)
}
