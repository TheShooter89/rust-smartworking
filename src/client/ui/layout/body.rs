use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

//use crate::client::ui::pages::{LoginPage, PageFrame};
use crate::client::ui::pages::{LoginPage, PageFrame};
use crate::client::ui::Layout::Navbar;
use crate::client::ui::{Component, Dashboard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<T: Component> {
    page: PageFrame<T>,
}

impl<T: Component> Component for Body<T> {
    fn render(&self) -> String {
        let result = html!(
            <body class="has-navbar-fixed-top">
                {Navbar::new().render()}
                {self.page.render()}
            </body>
        );

        result
    }
}

impl<T: Component> Body<T> {
    pub fn new(page: PageFrame<T>) -> Self {
        Body { page }
    }
}
