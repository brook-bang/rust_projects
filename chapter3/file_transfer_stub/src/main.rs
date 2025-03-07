use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, http::StatusCode, web, web::Path,
};
use std::fs::File;
use std::io::{self, Read, Write};

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

async fn delete_file(info: Pa)

fn main() {
    println!("Hello, world!");
}
