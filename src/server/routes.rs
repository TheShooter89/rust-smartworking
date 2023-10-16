use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

use html_to_string_macro::html;

use crate::client::ui::pages::{LoginPage, PageFrame};
use crate::client::ui::Layout::{Head, Page};
use crate::client::ui::{Component, Dashboard};

#[get("/")]
async fn home() -> Result<HttpResponse, Error> {
    let dash = Dashboard::new();
    let page = Page::new(PageFrame::new(dash));

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(page.render()))
}

#[get("/login")]
async fn login() -> Result<HttpResponse, Error> {
    let login = LoginPage::new();
    let page = Page::new(login);

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(page.render()))
}

pub fn routes_setup(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
    cfg.service(login);
}
