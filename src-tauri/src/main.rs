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
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu,Manager,SystemTray,SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
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

format!("local ip address: {ip}:{port}")
  
}

// pub async fn receive(
//     state: web::Data<AppState>
// ) -> impl Responder {
//     state.broadcaster.broadcast("Hdhdhdhdd").await;
//     HttpResponse::Ok().body("msg sent")
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct APIPayload {
    group: Option<String>,
    state:Option<String>,
    action:Option<String>,
}

// pub async fn receive(state: web::Data<AppState>,post: web::web::Json<Value>) -> impl Responder {
pub async fn receive(state: web::Data<AppState>,post: web::Json<APIPayload>) -> impl Responder {
// async fn receive(post: web::Json<APIPayload>) -> Result<HttpResponse, CustomError> {
    //  println!("Uploaded Content: {:#?}", uploaded_content);
    println!("Group {:#?}", post);

    // let state = req
    //     .app_data::<Data<AppState>>()
    //     .expect("app_data is empty!");

        // let app_state = state.get_ref();

        // println!("{:?}",state);

    //  tumira(state,"Hello World".to_string()).await;

    // convert json to string
    let json_str = to_string(&post).unwrap();

    // Broadcaster::broadcast("Hello HAHAHHAA");
    state.broadcaster.broadcast(&json_str).await;

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

        


        // rs2js("hello".to_string(),Manager);
        
        // tauri::invoke(shout("API"));
        // tauri::Invoke("shout", { Inv {phrase: e} });
        // tauri::Invoke("sfsafsdf", shout);
        // window.emit("ping", {}).unwrap();
        HttpResponse::Ok().json(post)
    }
    // Err(CustomError::BadClientData)

}

fn main() {
  
    thread::spawn(move || {
        let _ = start_server();
    });


    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
let close = CustomMenuItem::new("close".to_string(), "Close");
let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
let menu = Menu::new()
  .add_native_item(MenuItem::Copy)
  .add_item(CustomMenuItem::new("hide", "Hide"))
  .add_submenu(submenu);

// here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
let quit = CustomMenuItem::new("quit".to_string(), "Quit");
let hide = CustomMenuItem::new("hide".to_string(), "Hide");
let show = CustomMenuItem::new("show".to_string(), "Show");
let tray_menu = SystemTrayMenu::new()
  .add_item(quit)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(hide)
  .add_item(show);

//   let tray = SystemTray::new().with_menu(tray_menu);
    let system_tray = SystemTray::new()
    .with_menu(tray_menu);

    tauri::Builder::default()
        .menu(menu)
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            // SystemTrayEvent::MenuItemClick { id, .. } => {
            //     // get a handle to the clicked menu item
            //     // note that `tray_handle` can be called anywhere,
            //     // just get an `AppHandle` instance with `app.handle()` on the setup hook
            //     // and move it to another function or thread
            //     let item_handle = app.tray_handle().get_item(&id);
            //     match id.as_str() {
            //     "hide" => {
            //         let window = app.get_window("main").unwrap();
            //         window.hide().unwrap();
            //         // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
            //         item_handle.set_title("Show").unwrap();
            //     }
            //     "show" => {
            //         let window = app.get_window("main").unwrap();
            //         window.show().unwrap();
            //     }
            //     _ => {}
            //     }
            // }
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
                }
            }
            _ => {}
            })
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(3));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .invoke_handler(tauri::generate_handler![greet,shout,my_ip])
        // .invoke_handler(tauri::generate_handler![shout])
        // .invoke_handler(tauri::generate_handler![my_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

   

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
    let server_port = 9000;

   
   

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