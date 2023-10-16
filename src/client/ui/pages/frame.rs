use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageFrame<T: Component> {
    page: T,
}

impl<T: Component> Component for PageFrame<T> {
    fn render(&self) -> String {
        let result = html!({ &self.page.render() });

        result
    }
}

impl<T: Component> PageFrame<T> {
    pub fn new(page: T) -> Self {
        PageFrame { page }
    }
}
