// 执行顺序
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X GET http://localhost:8080/datafile.txt
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web, web::Path};
use futures_util::StreamExt;
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

async fn delete_file(info: Path<(String,)>) -> Result<HttpResponse, actix_web::Error> {
    let filename = &info.0;
    print!("Deleting file \"{}\" ... ", filename);
    flush_stdout();

    match std::fs::remove_file(filename) {
        Ok(_) => {
            println!("Deleted file \"{}\"", filename);
            Ok(HttpResponse::Ok().finish())
        }
        Err(error) => {
            println!("Failed to delete file \"{}\": {}", filename, error);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

async fn download_file(info: Path<(String,)>) -> Result<HttpResponse, actix_web::Error> {
    let filename = &info.0;
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();

    fn read_file_contents(filename: &str) -> std::io::Result<String> {
        use std::io::Read;
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    match read_file_contents(filename) {
        Ok(contents) => {
            println!("Downloaded file \"{}\"", filename);
            Ok(HttpResponse::Ok().content_type("text/plain").body(contents))
        }
        Err(error) => {
            println!("Failed to read file \"{}\": {}", filename, error);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

async fn upload_specified_file(
    mut payload: web::Payload,
    info: Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let filename = info.0.clone();
    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk?);
    }

    let f = File::create(&filename);
    if f.is_err() {
        println!("Failed to create file \"{}\"", filename);
        return Ok(HttpResponse::NotFound().finish());
    }

    if f.unwrap().write_all(&body).is_err() {
        println!("Failed to write file \"{}\"", filename);
        return Ok(HttpResponse::NotFound().finish());
    }

    println!("Uploaded file \"{}\"", filename);
    Ok(HttpResponse::Ok().finish())
}

async fn upload_new_file(
    mut payload: web::Payload,
    info: Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let filename_prefix = info.0.clone();
    print!("Uploading file \"{}*.txt\" ... ", filename_prefix);
    flush_stdout();

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk?);
    }

    let mut rng = rand::rng();
    let mut attempts = 0;
    let mut file;
    let mut filename;
    const MAX_ATTEMPTS: u32 = 100;

    loop {
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            println!(
                "Failed to create new file with prefix \"{}\", after {} attempts.",
                filename_prefix, MAX_ATTEMPTS
            );
            return Ok(HttpResponse::NotFound().finish());
        }

        filename = format!("{}{:03}.txt", filename_prefix, rng.random_range(0..1000)); // 更新为 random_range
        file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename);

        if file.is_ok() {
            break;
        }
    }

    if file.unwrap().write_all(&body).is_err() {
        println!("Failed to write file \"{}\"", filename);
        return Ok(HttpResponse::NotFound().finish());
    }

    println!("Uploaded file \"{}\"", filename);
    Ok(HttpResponse::Ok().content_type("text/plain").body(filename))
}

async fn invalid_resource(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    println!("Invalid URI: \"{}\"", req.uri());
    Ok(HttpResponse::NotFound().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {} ...", server_address);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_specified_file))
                    .route(web::post().to(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
    .await
}
