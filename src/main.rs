mod ports_open;
use std::collections::HashMap;
use ports_open::get_open_ports;
use actix_web::{get, post, web, App, HttpResponse, HttpServer,  Responder};
// use std::process::Command;
pub struct Uid();

#[get("/")]
async fn hello() -> impl Responder {
    // create a hashmap
    // let mut final_val:HashMap<String, String> = HashMap::new();
    let val = get_open_ports(1, 65000);
    let mut val_str = "The open ports are: ".to_string();
    val_str += &format!("{:?}", val).to_string();
    HttpResponse::Ok().body(val_str)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
