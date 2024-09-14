use std::sync::RwLock;
use actix_web::{HttpResponse, Responder, HttpRequest, web::Json};
use actix_web::post;
use actix_web::web::Data;
use log::info;
use serde_json::{Value, Map};
use serde::{Serialize,Deserialize};

use crate::{common::AppState, prolog};

#[derive(Debug, Serialize, Deserialize)]
pub struct RasaTracker {
	pub conversation_id: String,
    pub slots: Map<String, Value>,
    pub latest_message: Map<String, Value>,
    pub latest_event_time: f64,
	pub follup_up_action: String,
    pub paused: bool,
    pub events: Vec<Value>,
	pub latest_input_channel: String,
    pub active_loop: Value,
    pub latest_action: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RasaDomain {
    config: Value,
    session_config: Value,
    intents: Value,
    entities: Value,
    slots: Value,
    responses: Value,
    forms: Value,
    e2e_actions: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RasaMsg {
	pub next_action: String,
	pub sender_id: String,
	pub tracker: RasaTracker,
	pub domain: RasaDomain,
	pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RasaResponse {
    pub events: Vec<Value>,
    pub responses: Vec<Value>,
}

#[post("/rasa")]
pub async fn rasa(data: Data<RwLock<AppState>>, json: Json<Value>, _req: HttpRequest) -> impl Responder {
    info!("Api: Rasa");
    //let jstring = serde_json::to_string_pretty(&json).unwrap();
    let name = json.get("tracker").unwrap().get("latest_message").unwrap().get("intent").unwrap().get("name").unwrap(); 
    let data = data.write().unwrap();

    match name.as_str().unwrap() {
        "validate" => prolog::validate(&data),
        "change" => prolog::change(&data),
        "heal" => prolog::invalidate(&data),
        &_ => return HttpResponse::Ok()
    }


    return HttpResponse::Ok();
}
