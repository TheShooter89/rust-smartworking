use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Layout::Navbar;
use crate::client::ui::{Component, Dashboard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {}

impl Component for Body {
    fn render(&self) -> String {
        let result = html!(
            <body class="has-navbar-fixed-top">
                {Navbar::new().render()}
                {Dashboard::new().render()}
            </body>
        );

        result
    }
}

impl Body {
    pub fn new() -> Self {
        Body {}
    }
}
