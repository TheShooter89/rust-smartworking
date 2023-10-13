use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPanel {}

impl Component for ChatPanel {
    fn render(&self) -> String {
        let result = html!(
            <nav class="panel">
                <div class="panel-heading level">
                    <p>"CHATS"</p>
                    <button class="button is-warning">
                        <span class="icon">
                            <i class="fa fa-upload"></i>
                        </span>
                        <span>"IMPORT"</span>
                    </button>
                </div>
                <label class="panel-block ">
                    <input type="checkbox" />
                    "ANDREA (update: 23/10/03)"
                </label>
                <label class="panel-block ">
                    <input type="checkbox" />
                    "CARLA (update: 23/08/15)"
                </label>
                <div class="panel-block ">
                    <button class="button is-fullwidth">
                        "SHOW STATS"
                    </button>
                </div>
            </nav>
        );

        result
    }
}

impl ChatPanel {
    pub fn new() -> Self {
        ChatPanel {}
    }
}
