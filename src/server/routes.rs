use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::client::ui::{Component, Head, Page};

#[get("/")]
async fn home() -> Result<HttpResponse, Error> {
    let page = Page::new();

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(page.render()))
}

pub fn routes_setup(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}
