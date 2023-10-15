use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

mod globals;

mod client;
mod datalayer;
mod loader;
mod server;

use server::Server;

fn app_info() {
    println!("#################################################################");
    println!(
        "   // {}  // ({})",
        globals::APP_NAME.to_uppercase(),
        globals::APP_VERSION.to_lowercase()
    );
    println!("#################################################################");
    println!("application host: {:?}", globals::HOST);
    println!("application port: {:?}", globals::PORT);
    println!("application data folder: {:?}", globals::APP_DATA_ROOT);
    println!(
        "application database folder: {:?}",
        globals::DATABASE_FOLDER
    );
    println!("application database name: {:?}", globals::DATABASE_NAME);
    println!("application database path: {:?}", globals::DATABASE_PATH);
    println!("#################################################################");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, develop!\n");
    app_info();
    let app_server = Server::new();
    app_server.listen().await
}
