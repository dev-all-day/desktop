#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #![allow(unused)] // silence unused warnings while learning

mod routes;
mod errors;
mod custom_error_handler;
// mod ws;

use std::net::{TcpListener, Ipv4Addr, IpAddr};

use std::time::{SystemTime, UNIX_EPOCH};
use std::str;
use std::thread;
use actix_web::error::{InternalError, JsonPayloadError};
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
// use std::fs;
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::{web::Data,get,error, middleware, web, http, App, Error, HttpRequest, HttpResponse, HttpServer,Responder,dev,Result,body};
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
// use futures_util::StreamExt as _;
use serde_json::json;
use serde_json::{to_string, Value};
use json::JsonValue;
use port_scanner::local_ports_available_range;
use tauri::{AppHandle,CustomMenuItem, Menu, MenuItem, Submenu,Manager,SystemTray,SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent,WindowMenuEvent};
use actix_files::Files;
use handlebars::Handlebars;
use actix_cors::Cors;
use errors::error_handlers;
// mod err_handler;

// sse
mod broadcast;
use self::broadcast::Broadcaster;
use std::{io, sync::Arc};
use actix_web_lab::extract::Path;

// SSE 
pub struct  AppState{
    broadcaster:Arc<Broadcaster>
}

// SSE
pub async fn sse_client(state: web::Data<AppState>) -> impl Responder {
    println!("in api");
    state.broadcaster.new_client().await
}

pub async fn broadcast_msg(
    state: web::Data<AppState>,
    Path((msg,)): Path<(String,)>,
) -> impl Responder {
    state.broadcaster.broadcast(&msg).await;
    HttpResponse::Ok().body("msg sent")
}



// This command must be async so that it doesn't run on the main thread.
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust API!", name)
}

#[tauri::command]
fn shout(phrase: &str) -> String {
    format!("Hey, {}!", phrase)
}

#[tauri::command]
fn my_ip() -> String {
    // Check for open port
//    format!("{:?}",ip())
  let ip = if local_ip::get().is_some() {
    local_ip::get().unwrap()
}else{
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
};

// println!("{:?}",ip);

// //   let ip = local_ip::get().expect("failed to find local ip addr");
  let port = portpicker::pick_unused_port().expect("failed to find unused port");

format!("{ip}")
// format!("local ip address: {ip}:{port}")
  
}

// pub async fn receive(
//     state: web::Data<AppState>
// ) -> impl Responder {
//     state.broadcaster.broadcast("Hdhdhdhdd").await;
//     HttpResponse::Ok().body("msg sent")
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct APIPayload {
    connection: Option<String>,
    channel: Option<String>,
    session: Option<String>,
    state:Option<String>,
    value:Option<String>
}

// use json_patch::merge;

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

// pub async fn receive(state: web::Data<AppState>,post: web::web::Json<Value>) -> impl Responder {
pub async fn receive(state: web::Data<AppState>,post: web::Json<Value>) -> impl Responder {
// async fn receive(post: web::Json<APIPayload>) -> Result<HttpResponse, CustomError> {
    //  println!("Uploaded Content: {:#?}", uploaded_content);
    println!("Payload {:#?}", post);

    // let mut new_post = post.into_inner();
    // new_post.time = "hahaha";

    // let mut result = web::Json(new_post);
    // result.merge(post);

    let now = Local::now();
    let log_date= now.format("%d-%m-%Y").to_string();
    let log_time = now.format("%H:%M:%S%.3f").to_string();

    let new_json = json!({ "date": log_date,"time": log_time });
    let mut merged_json = post.into_inner();
    // result.merge(new_json);

    // let merged_json = merge(result, &new_json);

    // let new_data = json!({ "key": "value" });
    // merged_json.merge(new_json);

    merge(&mut merged_json, &new_json);
    

    // let state = req
    //     .app_data::<Data<AppState>>()
    //     .expect("app_data is empty!");

        // let app_state = state.get_ref();

        // println!("{:?}",state);

    //  tumira(state,"Hello World".to_string()).await;

    // convert json to string
    let json_str = to_string(&merged_json).unwrap();

    // Broadcaster::broadcast("Hello HAHAHHAA");
    state.broadcaster.broadcast(&json_str).await;

    // match post {
    //     Ok(post) => HttpResponse::Ok().json(post),
    //     // _ => HttpResponse::BadRequest().body("Invalid Request")
    //     _ => Err(CustomError::BadClientData)
    // }

    // let connection = post.connection.to_owned();

    // // let window: &Window;

    // if connection.is_none() {
    //     HttpResponse::BadRequest().body(json!({
    //         "code": 400,
    //         "message": "Invalid request",
    //         "payload" : {
    //             "error":"connection is required"
    //         }
    //     })
    //     .to_string())
    // }else{

        


    //     // rs2js("hello".to_string(),Manager);
        
    //     // tauri::invoke(shout("API"));
    //     // tauri::Invoke("shout", { Inv {phrase: e} });
    //     // tauri::Invoke("sfsafsdf", shout);
    //     // window.emit("ping", {}).unwrap();
    //     HttpResponse::Ok().json(post)
    // }
    // // Err(CustomError::BadClientData)

    HttpResponse::Ok().json(merged_json)

}

use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

#[cfg(target_os = "macos")]
use tauri::AboutMetadata;




mod app;
mod conf;
mod utils;

use app::{menu,setup};
use conf::ChatConfJson;

fn main() {
  
    thread::spawn(move || {
        let _ = start_server();
    });

    let context = tauri::generate_context!();

    let chat_conf = ChatConfJson::get_chat_conf();

    let mut builder = tauri::Builder::default()
        .menu(menu::init())
        .system_tray(menu::tray_menu())
        .on_menu_event(menu::menu_handler)
        .on_system_tray_event(menu::tray_handler)
        .on_window_event(|event| {
            // https://github.com/tauri-apps/tauri/discussions/2684
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                let win = event.window();
                if win.label() == "main" {
                    // TODO: https://github.com/tauri-apps/tauri/issues/3084
                    // event.window().hide().unwrap();
                    // https://github.com/tauri-apps/tao/pull/517
                    #[cfg(target_os = "macos")]
                    event.window().minimize().unwrap();

                    // fix: https://github.com/lencx/ChatGPT/issues/93
                    #[cfg(not(target_os = "macos"))]
                    event.window().hide().unwrap();
                } else {
                    win.close().unwrap();
                }
                api.prevent_close();
            }
        })
        .setup(setup::init);
        // .invoke_handler(tauri::generate_handler![shout])
        // .invoke_handler(tauri::generate_handler![my_ip])
        // .run(context)
        // .expect("error while running {dev.all.day} application");

    // if chat_conf.tray {
    //     builder = builder.system_tray(menu::tray_menu());
    // }

    builder
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .invoke_handler(tauri::generate_handler![greet,shout,my_ip])
        .run(context)
        .expect("error while running {dev.all.day} application");

}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[actix_rt::main]
async fn start_server() -> std::io::Result<()> {

    let broadcaster = Broadcaster::create();

    let template_service = {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".html", "src/web/templates")
            .unwrap();

        Data::new(handlebars)
    };

    let server_addr = "127.0.0.1";
    let server_port = 3310;

   
   

    let app = HttpServer::new(move || {

        let state = web::Data::new(AppState {
            broadcaster: Arc::clone(&broadcaster)
        });

        // let cors = Cors::default()
        let cors = Cors::default()
            // .allowed_origin("//localhost")
            // .allowed_origin("//127.0.0.1")
            // .send_wildcard()
            // .allowed_origin("*")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            // .wrap(error_handlers())
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .app_data(template_service.clone())
            .app_data(state.clone())
            // .app_data(web::Data::new(AppState {
            //     broadcaster: Arc::clone(&broadcaster)
            // }))
            .service(Files::new("/public", "src/web/public").show_files_listing())
            .service(routes::index)
            // .service(web::resource("/").route(web::get().to(HttpResponse::InternalServerError)))
            .service(routes::ip)
            .service(getPort)
            .service(routes::test)
            // .service(routes::receive)
            .service(
                web::resource("/")
                .app_data(
                    web::JsonConfig::default()
                        // .limit(4096)
                        .error_handler(post_error)
                )
                .route(web::post().to(receive))
            )
             // This route is used to listen events/ sse events
            .route("/events", web::get().to(sse_client))
            // .route("/events{_:/?}", web::get().to(sse_client))
            // This route will create notification
            .route("/events/{msg}", web::get().to(broadcast_msg))
            // .route("/", web::get().to(compliment))
            // .route("/ip", web::get().to(ip))
    })
    // .workers(2)
    .bind((server_addr, server_port))?
    .run();
    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}

#[derive(Serialize)]
struct PostError {
    status_code: i32,
    message: String,
    error: String,
}
    fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
     
      let post_error = PostError {
        status_code: 400,
        message: "Invalid request".to_string(),
        error: format!("{}", err),
      };

      InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
    }

use chrono::{Local, DateTime};

#[get("/port")]
// pub async fn ip() -> Result<HttpResponse, Error> {
pub async fn getPort() -> impl Responder {


    let mut special_port1 = 0;

  // Check for open port
    if let Some(available_port) = get_available_port() {
        // println!("port `{}` is available", available_port);
        special_port1 = available_port;
    }

    let mut special_port = 0;

    for available in local_ports_available_range(3000..3005) {
        // println!("Port {} is available to use", available);
        special_port = available;
        break;
        // if special_port == 0 {
        //     special_port = available;
        //     break;
        // }else {
        //     break;
        // }
    }


    let now = Local::now();
    let formatted_time = now.format("%d-%m-%Y %H:%M:%S%.3f").to_string();


     HttpResponse::Ok().content_type("application/json").body(
        json!({
            "port1": format!("Port {} is available to use", special_port1),
            "port2": format!("Port {} is available to use", special_port),
            "time": format!("{}", formatted_time),
        })
        .to_string(),
    )

}

pub async fn ip() -> impl Responder {
  let ip = local_ip::get().unwrap();
  let port = portpicker::pick_unused_port().expect("failed to find unused port");

  format!("local ip address: {ip}:{port}")
}

fn get_available_port() -> Option<u16> {
    (3000..9000).find(|port| port_is_available(*port))
}

fn port_is_available(port:u16) -> bool {
    match TcpListener::bind(("127.0.0.1",port)){
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn homepage() -> impl Responder {
    format!("Welcome!")
}