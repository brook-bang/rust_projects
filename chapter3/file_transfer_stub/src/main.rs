use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, http::StatusCode, web, web::Path,
};
use std::fs::File;
use std::io::{self, Read, Write};

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

async fn delete_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Deletingfile \"{}\" ... ", filename);

    match std::fs::remove_file(filename) {
        Ok(()) => println!("Deleted file \"{}\"", filename),
        Err(_) => println!("Failed to delete file \"{}\"", filename),
    }
    HttpResponse::Ok().finish()
}

async fn download_file(info: Path<(String,)>) -> Result<HttpResponse> {
    let filename = &info.0;
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("Failed to open file \"{}\"", filename);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        println!("Failed to read file \"{}\"", filename);
        return Ok(HttpResponse::InternalServerError().finish());
    }

    println!("Downloaded file \"{}\"", filename);
    Ok(HttpResponse::Ok().content_type("text/plain").body(contents))
}

async fn upload_specified_file(info: Path<(String,)>, body: String) -> Result<HttpResponse> {
    let filename = &info.0;
    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    let contents = body;

    match File::create(filename) {
        Ok(mut file) => {
            if file.write_all(contents.as_bytes()).is_err() {
                println!("Failed to write to file \"{}\"", filename);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
        Err(_) => {
            println!("Failed to create file \"{}\"", filename);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    }
    println!("Uploaded file \"{}\"", filename);
    Ok(HttpResponse::Ok().finish())
}

async fn upload_new_file(info: Path<(String,)>, body: String) -> Result<HttpResponse> {
    let filename_prefix = &info.0;
    print!("Uploading file \"{}*.txt\" ... ", filename_prefix);
    flush_stdout();

    let contents = body;

    let file_id = 17;
    let filename = format!("{}{}.txt", filename_prefix, file_id);

    match File::create(&filename) {
        Ok(mut file) => {
            if file.write_all(contents.as_bytes()).is_err() {
                println!("Failed to write to file \"{}\"", filename);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
        Err(_) => {
            println!("Failed to create file \"{}\"", filename);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    }

    println!("Uploaded file \"{}\"", filename);
    Ok(HttpResponse::Ok().content_type("text/plain").body(filename))
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::build(StatusCode::NOT_FOUND).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8000";
    println!("Listening at address {} ...", server_address);

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_new_file))
                    .route(web::post().to(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
    .await
}
