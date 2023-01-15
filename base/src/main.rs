mod client;

use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

use attributes::slash_command;
use client::Client;

const TOKEN: &str = "MTA1ODc2MzQ0ODYyNTkzODUwNA.GRBUIj.LLQwKc8YKfbSa19kggS4_PTNkggz8V089ZqLUM";
const APP_ID: u64 = 1058763448625938504;

#[post("/")]
async fn my_command(req_body: String) -> impl Responder {
    let mut resp = HashMap::new();
    resp.insert("type", 1);
    HttpResponse::Ok().json(resp)
}

#[slash_command(name = "blep", description = "A slash command")]
fn blep() {
    println!("blep");
}

// #[actix_web::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new(APP_ID);

    client.start().await;

    HttpServer::new(|| App::new().service(my_command))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
