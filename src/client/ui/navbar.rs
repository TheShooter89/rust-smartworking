use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navbar {}

impl Component for Navbar {
    fn render(&self) -> String {
        let result = html!(
            <nav class="navbar is-fixed-top cw-navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item">
                        <img src="static/img/cluexis-logo.png" />
                    </a>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <h1 class="subtitle navbar-item cw-navbar-apptitle">"cluexis-web"</h1>
                    </div>

                    <div class="navbar-end">
                        <div class="navbar-item">
                            <span class="tag is-info">"v0.1.0-alpha"</span>
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
