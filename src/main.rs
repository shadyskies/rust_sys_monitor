mod ports_open;
// use std::collections::HashMap;
use sysinfo::{DiskExt, NetworkExt, ProcessExt, System, SystemExt};
use ports_open::get_open_ports;
use actix_web::{get, post, web, App, HttpResponse, HttpServer,  Responder};
use serde::{Deserialize, Serialize};
use tera::{Tera, Context};


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
    }
    web::Json(vec)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Rust Title");
    data.insert("name","Shadyskies");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn hardware_test(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let sys = System::new_all();

    println!("=> disks:");
    let mut disks_name = Vec::new();
    let mut disks_fs = Vec::new();
    let mut disks_type = Vec::new();
    let mut disks_mount = Vec::new();
    let mut disks_available = Vec::new();
    
    for disk in sys.disks() {
        println!("{:?}\n", disk.name());
        disks_name.push(disk.name().to_str());
        let tmp = disk.file_system();
        disks_fs.push(tmp);
        disks_type.push(disk.type_());
        disks_mount.push(disk.mount_point().to_str());
        disks_available.push(disk.available_space());
    }

    data.insert("disks_name", &disks_name);
    data.insert("disks_fs", &disks_fs);
    // data.insert("disks_type", &disks_type);
    // println!("{:?}", disks_vec);
    let rendered = tera.render("hardware.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/pid_list", web::get().to(get_pid_list))
            .route("/index", web::get().to(index))
            .route("/hardware_test", web::get().to(hardware_test))

        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
