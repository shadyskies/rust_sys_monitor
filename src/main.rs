mod ports_open;
// use std::collections::HashMap;
use sysinfo::{DiskExt, NetworkExt, ProcessExt, ProcessorExt, System, SystemExt, RefreshKind};
use ports_open::get_open_ports;
use actix_web::{get, post, web, App, HttpResponse, HttpServer,  Responder};
use serde::{Deserialize, Serialize};
use tera::{Tera, Context};
use std::str;


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

    // disk info
    println!("=> disks:");
    let mut disks_name = Vec::new();
    let mut disks_fs = Vec::new();
    let mut disks_type = Vec::new();
    let mut disks_mount = Vec::new();
    let mut disks_available = Vec::new();
    let mut disk_total = Vec::new();


    for disk in sys.disks() {
        disks_name.push(disk.name().to_str());
        let s = match str::from_utf8(disk.file_system()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        disks_fs.push(s);
        
        disks_type.push(disk.type_());
        disks_mount.push(disk.mount_point().to_str());
        disks_available.push(disk.available_space() / 1000000);
        disk_total.push(disk.total_space() / 1000000);
    }

    // sys info:
    data.insert("tot_mem", &sys.total_memory());
    data.insert("used_mem", &sys.used_memory());
    data.insert("tot_swap", &sys.total_swap());
    data.insert("used_swap", &sys.used_swap());
    data.insert("num_processors", &sys.processors().len());


    data.insert("disks_name", &disks_name);
    data.insert("disks_fs", &disks_fs);
    // data.insert("disks_type", &disks_type);
    data.insert("disks_mount", &disks_mount);
    data.insert("disks_available", &disks_available);
    data.insert("disks_total", &disk_total);
    
    let rendered = tera.render("hardware.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// TODO: processor usage shows 0 
#[get("/api/cpu/")]
async fn get_cpus() -> impl Responder {
    let s = System::new_with_specifics(RefreshKind::new().with_cpu());
    let mut _usage = Vec::new();
    for processor in s.processors() {
        println!("{}%", processor.cpu_usage());
        _usage.push(processor.cpu_usage());
    }
    for processor in s.processors() {
        println!("{}%", processor.cpu_usage());
        // _usage.push(processor.cpu_usage());
    }
    web::Json(_usage)
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
            .service(get_cpus)
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
