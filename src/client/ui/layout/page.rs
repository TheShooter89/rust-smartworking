use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::globals;

use crate::client::ui::pages::{LoginPage, PageFrame};
use crate::client::ui::Layout::{Body, Head};
use crate::client::ui::{Component, Dashboard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T: Component> {
    root: T,
}

impl<T: Component + Clone> Component for Page<T> {
    fn render(&self) -> String {
        let start_raw = "<!DOCTYPE html>\n<html lang=en>";
        let end_raw = "\n</html>";
        let head = Head::new(globals::APP_NAME).render();
        let content = PageFrame::new(self.root.clone());
        let body = Body::new(content).render();

        let result = start_raw.to_string() + head.as_str() + body.as_str() + end_raw;
        result
    }
}

impl<T: Component> Page<T> {
    pub fn new(root: T) -> Self {
        Page { root }
    }
}
