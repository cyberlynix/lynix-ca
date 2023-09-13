use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::CloseReason;
use serde::{Deserialize, Serialize};
use serde_json::from_str;


// Websocket
pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::new(), &req, stream);
    println!("{:?}", resp);
    resp
}

/// Define HTTP actor
struct MyWs {

}

impl MyWs {
    fn new() -> Self {
        MyWs {

        }
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Deserialize)]
pub struct AuthExchange {
    client_type: String,
    client_id: String,
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(client_info) = serde_json::from_str::<AuthExchange>(&text) {
                    ctx.text(format!("LynxWS: Client {} authentificated succesfully.", client_info.client_id));
                } else {
                    if let Ok(client_info) = from_str::<serde_json::Value>(&text) {
                        // Successfully parsed as serde_json::Value.
                        // You can now access the JSON data and check intensity.
                        if let Some(intensity) = client_info.get("intensity") {
                            // Use the intensity value here.
                            ctx.text(format!("LynxWS: Intensity is {}", intensity));
                        } else {
                            // Intensity key is missing in the JSON.
                            ctx.text("LynxWS: Error - Intensity not found in the JSON.");
                        }
                    } else {
                        // Parsing as serde_json::Value also failed.
                        ctx.text("LynxWS: Error - Invalid JSON format.");
                    }
                }
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                ctx.text("LynxWS: Unsupported Data Type.");
            },
        }
    }
}