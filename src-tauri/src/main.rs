#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #![allow(unused)] // silence unused warnings while learning

mod routes;
// mod ws;

use std::net::{TcpListener, Ipv4Addr, IpAddr, TcpStream};

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
// mod err_handler;

pub struct  AppState{
    server_running: bool,
    window: Window
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

#[tauri::command]
async fn change_window_title(window: tauri::Window,title: String) {

    if let Some(main) = window.get_window("main") {
        let new_title = title.clone();
        main.set_title(&new_title).unwrap();
    }

}


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
    let ip = if local_ip::get().is_some() {
        local_ip::get().unwrap()
    }else{
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    };

    // format!("{ip}")
    format!("127.0.0.1")
  
}

// PORT
fn is_port_in_use(port: u16) -> bool {
    match TcpStream::connect(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

use tauri::Window;


fn send_message(window: Window, event: &str, payload: &str) {
    window.emit(event, payload).unwrap();
}

#[tauri::command]
fn do_some_long_task(window: Window){
    let mut dd: String = String::from("Hello World");
    window.emit("PROGRESS", dd).unwrap();
}

#[tauri::command]
fn is_server_running(window: Window) -> bool{
    // let state = window.state().clone();
    // .server_running
    true
}

#[tauri::command]
fn start_my_server(window: Window){
    thread::spawn(move || {
        let _ = start_server(window);
    });
}

fn available_port(port: u16) -> u16 {

    let mut port = port;

    if is_port_in_use(port) {
        // Port is already in use.
        // print!("Port {port} is already in use.");
        loop {
            let addr = format!("127.0.0.1:{}", port);
            match TcpStream::connect(&addr) {
                Ok(_) => {
                    port += 1;
                },
                Err(_) => {
                    // Another Port is available!
                    // print!("Another port {port} is available!");
                    return port;
                }
            }
        }

    } else {
        // Port is available.
        // print!("Port {port} is available.");
        port

    }

}

fn get_open_port(starting_port:u16) -> String {
    let port = available_port(starting_port);
    format!("{port}")
}

#[tauri::command]
fn my_port() -> String {
    get_open_port(3000)
  
}

#[tauri::command]
fn cmd_get_config() -> ChatConfJson {
    ChatConfJson::get_chat_conf()
  
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

    let now = Local::now();
    let log_date= now.format("%d-%m-%Y").to_string();
    let log_time = now.format("%H:%M:%S%.3f").to_string();

    let new_json = json!({ "date": log_date,"time": log_time });
    let mut merged_json = post.into_inner();
    
    merge(&mut merged_json, &new_json);

    // convert json to string
    let json_str = to_string(&merged_json).unwrap();

    // let mut dd: String = String::from("Hello World");
    state.window.emit("PROGRESS", &json_str).unwrap();
    // send_message(state.window.clone(),"PROGRESS", &json_str);

    HttpResponse::Ok().json(merged_json)

}

use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

#[cfg(target_os = "macos")]
use tauri::AboutMetadata;

mod app;
mod conf;
mod utils;

use app::{menu,setup,cmd};
use conf::ChatConfJson;


fn main() {
  
    // thread::spawn(move || {
    //     let _ = start_server();
    // });

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
        .invoke_handler(tauri::generate_handler![
            greet,
            shout,
            my_ip,
            my_port,
            cmd_get_config,
            do_some_long_task,
            start_my_server,
            is_server_running,
            change_window_title,
            cmd::run_check_update,
        ])
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
async fn start_server(window: Window) -> std::io::Result<()> {

    let template_service = {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".html", "src/web/templates")
            .unwrap();

        Data::new(handlebars)
    };

    let server_addr = my_ip();
    // let server_port = available_port(3000);
    // let server_port = u16::from_str(my_port()).unwrap();
    // convert String to u16
    // let mut server_port = my_port().parse().unwrap_or(0);
    let mut server_port = 0;

    let state = web::Data::new(AppState {
        server_running: false,
        window
    });

    // server_port = state.port;

    let chat_conf = ChatConfJson::get_chat_conf();
    server_port = chat_conf.port;


    let app = HttpServer::new(move || {

        let cors = Cors::default()
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
            .service(Files::new("/public", "src/web/public").show_files_listing())
            .service(routes::index)
            // .service(web::resource("/").route(web::get().to(HttpResponse::InternalServerError)))
            // .service(routes::receive)
            .service(
                web::resource("/")
                .app_data(
                    web::JsonConfig::default()
                        // .limit(4096)
                        .error_handler(post_error)
                )
                .route(web::post().to(receive))
                // .route(web::post().to(|item: web::Data<AppState>| receive(state,item,window))
            )
             // This route is used to listen events/ sse events
            // This route will create notification
            // .route("/", web::get().to(compliment))
            // .route("/ip", web::get().to(ip))
    })
    // .workers(2)
    .bind((server_addr, server_port))?
    .run();
    println!("Server running at http://127.0.0.1:{}/",chat_conf.port);
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
