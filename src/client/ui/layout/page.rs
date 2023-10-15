use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::globals;

use crate::client::ui::Component;
use crate::client::ui::Layout::{Body, Head};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {}

impl Component for Page {
    fn render(&self) -> String {
        let start_raw = "<!DOCTYPE html>\n<html lang=en>";
        let end_raw = "\n</html>";
        let head = Head::new(globals::APP_NAME).render();
        let body = Body::new().render();

        let result = start_raw.to_string() + head.as_str() + body.as_str() + end_raw;
        result
    }
}

impl Page {
    pub fn new() -> Self {
        Page {}
    }
}
