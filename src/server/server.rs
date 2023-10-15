use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::globals;

use crate::client::ui::Component;
use crate::client::ui::Layout::{Head, Page};
use crate::server::routes_setup;

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        return Server {};
    }

    pub async fn listen(&self) -> std::io::Result<()> {
        println!(
            "webui server running at {:?}:{:?}",
            globals::HOST,
            globals::PORT
        );
        HttpServer::new(|| {
            App::new()
                .service(
                    fs::Files::new("/static", "./static")
                        .show_files_listing()
                        .use_last_modified(true),
                )
                .configure(routes_setup)
        })
        .bind((globals::HOST, globals::PORT))?
        .run()
        .await
    }
}
