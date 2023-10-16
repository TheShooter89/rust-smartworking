use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::client::ui::pages::{LoginPage, PageFrame};
use crate::client::ui::Layout::{Head, Page};
use crate::client::ui::{Component, Dashboard};

#[post("/send_magic_link")]
async fn send_magic_link() -> Result<HttpResponse, Error> {
    let page = html!(
    <span>"Magic link sent"</span>
    );

    Ok(HttpResponse::Ok().content_type("text/html").body(page))
}

pub fn api_setup(cfg: &mut web::ServiceConfig) {
    cfg.service(send_magic_link);
}
