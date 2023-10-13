use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Home {}

impl Component for Home {
    fn render(&self) -> String {
        let result = html!(
            <section class="section is-small">
                <div class="container">
                    <h1 class="title">"Hello actix webserver!"</h1>
                    <h2 class="subtitle">"actix-web + htmx + rusqlite = <3"</h2>
                </div>
            </section>
        );

        result
    }
}

impl Home {
    pub fn new() -> Self {
        Home {}
    }
}
