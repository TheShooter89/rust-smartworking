use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleStrip {
    titles: Vec<String>,
}

impl Component for TitleStrip {
    fn render(&self) -> String {
        let titles_string = match self.titles.len() {
            0 => "NO TITLES PROVIDED".to_string(),
            1 => self.titles[0].to_uppercase().clone().clone(),
            _ => self.titles.join(" - ").to_uppercase(),
        };

        let result = html!(
            <header class="box cw-titlestrip">
                <h1 class="title cw-titlestrip-titles">
                    {titles_string}
                </h1>
            </header>
        );

        result
    }
}

impl TitleStrip {
    pub fn new() -> Self {
        TitleStrip { titles: Vec::new() }
    }

    pub fn add(&mut self, title: &str) -> &mut Self {
        self.titles.push(title.to_string());
        self
    }
}
