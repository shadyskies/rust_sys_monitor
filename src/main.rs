mod ports_open;
// use std::collections::HashMap;
use sysinfo::{ProcessExt, System, SystemExt};
use ports_open::get_open_ports;
use actix_web::{get, post, web, App, HttpResponse, HttpServer,  Responder};
use serde::{Deserialize, Serialize};
// use std::process::Command;
pub struct Uid();

#[derive(Serialize)]
struct PID{
    pid_val: i32,
    process: String
}

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

async fn get_pid_list()  -> impl Responder {
    let mut vec = Vec::new();
    let s = System::new_all();
    for (pid, processval) in s.processes() {
        vec.push(PID{process: processval.name().to_string(), pid_val: *pid});
        println!("{}::{:?}", pid, processval.name());
    }
    vec.push(PID{process: "Redis Server".to_string(), pid_val: 6379});
    web::Json(vec)
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
            .route("/list", web::get().to(get_pid_list))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
