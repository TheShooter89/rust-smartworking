use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPage {}

impl Component for LoginPage {
    fn render(&self) -> String {
        let result = html!(
            <section class="section is-small">
                <div class="container">
                    <h1 class="title">"Hello Login Page"</h1>
                    <h2 class="subtitle">"here you'll be able to login via a magic link very soon"</h2>
                </div>
            </section>
        );

        result
    }
}

impl LoginPage {
    pub fn new() -> Self {
        LoginPage {}
    }
}
