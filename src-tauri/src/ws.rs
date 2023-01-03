// use actix::{prelude::*, StreamHandler};
// use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use actix_web_actors::ws;
// use actix::Actor;

// struct Ws;

// impl Actor::Actor for Ws {
//     type Context = ws::WebsocketContext<Self>;
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => ctx.text(text),
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }