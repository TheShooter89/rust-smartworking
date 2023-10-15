use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::globals;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navbar {}

impl Component for Navbar {
    fn render(&self) -> String {
        let result = html!(
            <nav class="navbar is-fixed-top cw-navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item">
                        <img src="static/img/rsw_logo.png" />
                    </a>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <h1 class="subtitle navbar-item cw-navbar-apptitle">{globals::APP_NAME}</h1>
                    </div>

                    <div class="navbar-end">
                        <div class="navbar-item">
                            <span class="tag is-info">"v"{globals::APP_VERSION}</span>
                        </div>
                    </div>
                </div>
            </nav>
        );

        result
    }
}

impl Navbar {
    pub fn new() -> Self {
        Navbar {}
    }
}
