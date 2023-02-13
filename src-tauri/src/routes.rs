use std::time::Instant;

use actix_files::NamedFile;
use actix_web::{web::Data,get, post, web, Error, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use handlebars::Handlebars;


// use tauri::{Window, window};

use gloo_console::{log, __macro::JsValue};

#[derive(Serialize)]
struct Person {
    name: &'static str,
    job: &'static str,
}

#[get("/")]
// async fn index() -> impl Responder {
async fn index(hb: Data<Handlebars<'_>>) -> impl Responder {

    // tumira(state,"Hello World".to_string()).await;

    // let req = HttpRequest::default();
    // let resp = app.call(req).await.unwrap();


    let data = Person {
        name: "Garikai Jenje",
        job: "Software Developer",
    };
    // let mut hb = Handlebars::new();
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

// use tauri::{Event};
// use tauri::event::EventResult;

// fn send_message_to_frontend() -> EventResult {
//     let message = "Hello from the backend!";
//     let event = Event::new("send-message-to-frontend", Some(message));
//     event.fire()
// }

// use tauri::{Manager};
// use tauri::WebView;


// // A function that sends a message from Rust to JavaScript via a Tauri Event
// fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
//     // info!(?message, "rs2js");
//     manager
//         .emit_all("rs2js", message)
//         .unwrap();
// }

// struct SomeStruct {
// field: i32
// }

// fn send_event(webview: &mut WebView<()>) {
//     let value = SomeStruct { field: 42 };
//     let _ = webview.emit("my-event", &value);
// }

// use self::broadcast::Broadcaster;
// use crate::broadcast;
use std::{sync::Arc};
// pub struct  AppState{
//     broadcaster:Arc<Broadcaster>,
// }
// pub async fn tumira(
//     state: web::Data<AppState>,
//     msg: String,
// ) -> impl Responder {
//     state.broadcaster.broadcast(&msg).await;
//     HttpResponse::Ok().body("msg sent")
// }
// #[post("/")]
// pub async fn receive(req: HttpRequest,post: web::Json<APIPayload>) -> impl Responder {

// SSE 


pub async fn receiveeee(post: web::Json<APIPayload>) -> impl Responder {
// async fn receive(post: web::Json<APIPayload>) -> Result<HttpResponse, CustomError> {
    //  println!("Uploaded Content: {:#?}", uploaded_content);
    println!("Group {:#?}", post);

    // let state = req
    //     .app_data::<Data<AppState>>()
    //     .expect("app_data is empty!");

        // let app_state = state.get_ref();

        // println!("{:?}",state);

    //  tumira(state,"Hello World".to_string()).await;

    // Broadcaster::broadcast("Hello HAHAHHAA");

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

