use std::time::Instant;

use actix_files::NamedFile;
use actix_web::{web::Data,get, post, web, Error, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use handlebars::Handlebars;


use crate::{custom_error_handler::CustomError, shout};
use tauri::{Window, window};

use gloo_console::{log, __macro::JsValue};

#[derive(Serialize)]
struct Person {
    name: &'static str,
    job: &'static str,
}


#[get("/test")]
pub async fn test() -> impl Responder {
    let object = JsValue::from("any JsValue can be logged");
    log!("text", object);
    format!("Welcome!")
}

#[get("/ip")]
// pub async fn ip() -> Result<HttpResponse, Error> {
pub async fn ip() -> impl Responder {

  let ip = local_ip::get().unwrap();
  let port = portpicker::pick_unused_port().expect("failed to find unused port");

    // match ip && port {
    //     Ok(_) => HttpResponse::Ok().json(post),
    //     _ => HttpResponse::BadRequest().body("Error trying to create a new post")
    // }

  // format!("local ip address: {ip}:{port}")
    HttpResponse::Ok().content_type("application/json").body(
        json!({
            "status": "Success",
            "code": 200,
            "message": format!("local ip address: {ip}:{port}"),
            "ip":ip,
            "port":port
        })
        .to_string(),
    )
    // Ok(res)
}

#[get("/")]
async fn index(hb: Data<Handlebars<'_>>) -> impl Responder {

    let data = Person {
        name: "Garikai Jenje",
        job: "Software Developer",
    };
let html = hb.render("compliment", &data).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
    // format!("Welcome!")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIPayload {
    group: Option<String>,
    state:Option<String>,
    action:Option<String>,
}

struct Ping {
    data: String
}

// #[post("/")]
pub async fn receive(post: web::Json<APIPayload>) -> impl Responder {
// async fn receive(post: web::Json<APIPayload>) -> Result<HttpResponse, CustomError> {
    //  println!("Uploaded Content: {:#?}", uploaded_content);
    println!("Group {:#?}", post);

    // match post {
    //     Ok(post) => HttpResponse::Ok().json(post),
    //     // _ => HttpResponse::BadRequest().body("Invalid Request")
    //     _ => Err(CustomError::BadClientData)
    // }

    let group = post.group.to_owned();

    // let window: &Window;

    if group.is_none() {
        HttpResponse::BadRequest().body(json!({
            "code": 400,
            "message": "Invalid request",
            "payload" : {
                "error":"group is required"
            }
        })
        .to_string())
    }else{
        
        // tauri::invoke(shout("API"));
        // tauri::Invoke("shout", { Inv {phrase: e} });
        // tauri::Invoke("sfsafsdf", shout);
        // window.emit("ping", {}).unwrap();
        HttpResponse::Ok().json(post)
    }
    // Err(CustomError::BadClientData)

}

