use crate::{conf::ChatConfJson, utils};
use tauri::{utils::config::WindowUrl, window::WindowBuilder, App, GlobalShortcutManager, Manager};
use wry::application::accelerator::Accelerator;

use log::info;



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
use actix_web::{web::Data,get,error, middleware, web, http, App as ActixApp, HttpRequest, HttpResponse, HttpServer,Responder,dev,Result,body};
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
// use futures_util::StreamExt as _;
use serde_json::json;
use serde_json::{to_string, Value};
use json::JsonValue;
use port_scanner::local_ports_available_range;
use tauri::{AppHandle,CustomMenuItem, Menu, MenuItem, Submenu,SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent,WindowMenuEvent};
use actix_files::Files;
use handlebars::Handlebars;
use actix_cors::Cors;
// mod err_handler;

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let theme = ChatConfJson::theme();
    let chat_conf = ChatConfJson::get_chat_conf();
    let handle = app.app_handle();

    // tauri::async_runtime::spawn(async move {
    //     window::tray_window(&handle);
    // });

    if let Some(v) = chat_conf.global_shortcut {
        // info!("global_shortcut: `{}`", v);
        match v.parse::<Accelerator>() {
            Ok(_) => {
                // info!("global_shortcut_register");
                let handle = app.app_handle();
                let mut shortcut = app.global_shortcut_manager();
                shortcut
                    .register(&v, move || {
                        if let Some(w) = handle.get_window("main") {
                            if w.is_visible().unwrap() {
                                w.hide().unwrap();
                            } else {
                                w.show().unwrap();
                                w.set_focus().unwrap();
                            }
                        }
                    })
                    .unwrap_or_else(|err| {
                        info!("global_shortcut_register_error: {}", err);
                    });
            }
            Err(err) => {
                info!("global_shortcut_parse_error: {}", err);
            }
        }
    } else {
        info!("global_shortcut_unregister");
    };

    let app = app.handle();

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


    // mount the web server instance
    // tauri::async_runtime::spawn(async move {
    //     let _actix = start_server();
    // });
    
    // thread::spawn(move || {
    //     let _actix = start_server();
    // });
        

    // auto_update
    if chat_conf.auto_update != "Disable" {
        info!("stepup::run_check_update");
        // let app = app.handle();
        utils::run_check_update(app, chat_conf.auto_update == "Silent", None);
    }

    Ok(())
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

    let server_addr = String::from("127.0.0.1:");
    // let server_port = available_port(3000);
    // let server_port = u16::from_str(my_port()).unwrap();
    // convert String to u16
    // let mut server_port = my_port().parse().unwrap_or(0);
    let mut server_port = 0;

    

    let chat_conf = ChatConfJson::get_chat_conf();
    server_port = chat_conf.port;



    let app = HttpServer::new(move || {

        


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

        ActixApp::new()
            .wrap(cors)
            // .wrap(error_handlers())
            // .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .app_data(template_service.clone())
            // .app_data(web::Data::new(AppState {
            //     broadcaster: Arc::clone(&broadcaster)
            // }))
            .service(Files::new("/public", "src/web/public").show_files_listing())
            // .service(routes::index)
            // .service(web::resource("/").route(web::get().to(HttpResponse::InternalServerError)))
            // .service(routes::receive)
            // .service(
            //     web::resource("/")
            //     .app_data(
            //         web::JsonConfig::default()
            //             // .limit(4096)
            //             .error_handler(post_error)
            //     )
            //     .route(web::post().to(receive))
            // )
            // .route("/", web::get().to(compliment))
            // .route("/ip", web::get().to(ip))
    })
    // .workers(2)
    .bind((server_addr, server_port))?
    .run();
    println!("Server running at http://127.0.0.1:{}/",chat_conf.port);
    app.await
}