mod db_access;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web, web::Path};
use serde_derive::Deserialize;
use serde_json::json;
use std::sync::Mutex;

struct AppState {
    db: db_access::DbConnection,
}

async fn get_all_persons_ids(state: web::Data<Mutex<AppState>>) -> impl Responder {
    println!("In get_all_persons_ids");

    let db_conn = &state.lock().unwrap().db;
    let ids: Vec<u32> = db_conn.get_all_persons_ids().collect();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&json!(ids)).unwrap())
}

async fn get_person_name_by_id(
    state: web::Data<Mutex<AppState>>,
    info: Path<(String,)>,
) -> impl Responder {
    println!("In get_person_name_by_id");
    let id = &info.0;
    let id = match id.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    let db_conn = &state.lock().unwrap().db;
    match db_conn.get_person_name_by_id(id) {
        Some(name) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&json!(name)).unwrap()),
        None => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
pub struct Filter {
    partial_name: Option<String>,
}

async fn get_persons(
    state: web::Data<Mutex<AppState>>,
    query: web::Query<Filter>,
) -> impl Responder {
    println!("In get_persons");
    let db_conn = &state.lock().unwrap().db;
    let partial_name = query.partial_name.clone().unwrap_or_else(|| "".to_string());
    let persons: Vec<(u32, String)> = db_conn
        .get_persons_id_and_name_by_partial_name(&partial_name)
        .collect();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&json!(persons)).unwrap())
}

async fn insert_person(state: web::Data<Mutex<AppState>>, info: Path<(String,)>) -> impl Responder {
    println!("In insert_person");
    let name = &info.0;
    let db_conn = &mut state.lock().unwrap().db;
    let new_id = db_conn.insert_person(name);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&json!(new_id)).unwrap())
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {}", server_address);
    let db_conn = web::Data::new(Mutex::new(AppState {
        db: db_access::DbConnection::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(db_conn.clone())
            .service(web::resource("/persons/ids").route(web::get().to(get_all_persons_ids)))
            .service(
                web::resource("/person/name_by_id/{id}")
                    .route(web::get().to(get_person_name_by_id)),
            )
            .service(web::resource("/persons").route(web::get().to(get_persons)))
            .service(web::resource("/person/{name}").route(web::post().to(insert_person)))
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
    .await
}
