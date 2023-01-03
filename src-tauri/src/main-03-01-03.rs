#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #![allow(unused)] // silence unused warnings while learning

mod routes;
mod errors;
mod custom_error_handler;

use std::net::{TcpListener, Ipv4Addr, IpAddr};

use std::time::{SystemTime, UNIX_EPOCH};
use std::str;
use std::io;
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
use json::JsonValue;
use port_scanner::local_ports_available_range;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu,Manager,SystemTray,SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use actix_files::Files;
use handlebars::Handlebars;
use actix_cors::Cors;
use errors::error_handlers;
// mod err_handler;


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



fn main() {

    // Check for open port
    if let Some(available_port) = get_available_port() {
        println!("port `{}` is available", available_port);
    }

    let mut special_port = 0;

    for available in local_ports_available_range(3000..3005) {
        println!("Port {} is available to use", available);
        special_port = available;
        break;
        // if special_port == 0 {
        //     special_port = available;
        //     break;
        // }else {
        //     break;
        // }
    }

    thread::spawn(move || {
        let _ = start_server();
    });

    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let menu = Menu::new()
    // .add_native_item(MenuItem::Hide)
    // .add_native_item(MenuItem::Minimize)
    // .add_native_item(MenuItem::HideOthers)
    // .add_native_item(MenuItem::Separator)
    // .add_native_item(MenuItem::Quit)
    // .add_item(quit); // configure the menu

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

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![shout])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    /* Creating a Local TcpListener at Port 8477 */
    // const HOST : &str ="127.0.0.1";
    // const PORT : &str ="8477";
    // /* Concatenating Host address and Port to Create Final Endpoint */
    // let end_point : String = HOST.to_owned() + ":" +  PORT;
    // /*Creating TCP Listener at our end point */
    // let listener = TcpListener::bind(end_point).unwrap();
    // println!("Web server is listening at port {}",PORT);
    // /* Connecting to any incoming connections */
    // for stream in listener.incoming() {
    //     let _stream = stream.unwrap();
    //     println!("Connection established!");
    // }

    



}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
// fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    // Ok(ErrorHandlerResponse::Response(json!({
    //         "status": "Success",
    //         "code": 500,
    //         "message": "Something went wrong.",
    //     })
    //     .to_string()))
    // HttpResponse::Ok().content_type("application/json").body(
    //     json!({
    //         "status": "Success",
    //         "code": 200,
    //         "message": "Something went wrong."
    //     })
    //     .to_string(),
    // )
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    // let res = res.map_body(|_, _| body(from("{\"code\":413,\"error\":\"413 Payload Too Large\",\"message\":\"You've sent more data than expected\"}")).into_body());//alter the the response body see "https://users.rust-lang.org/t/actix-web-using-a-custom-error-handler-to-alter-the-response-body/41068"
    // Ok(ErrorHandlerResponse::Response(res))

    // let req = res.request();
    // // let res = res.map_body(|_, _| ResponseBody::Body(Body::from("test")));
    // let res = res.into_body();
    // Ok(ErrorHandlerResponse::Response(res))
}

#[actix_rt::main]
async fn start_server() -> std::io::Result<()> {

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
        let cors = Cors::default()
            .allowed_origin("//localhost")
            .allowed_origin("//127.0.0.1")
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            // .wrap(error_handlers())
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .app_data(template_service.clone())
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
                        .route(web::post().to(routes::receive))
                    )
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