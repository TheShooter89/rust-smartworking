use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Children<T>
where
    T: Component,
{
    list: Vec<T>,
}

impl<T: Component> Component for Children<T> {
    fn render(&self) -> String {
        let mut fin_result = String::new();

        for child in &self.list {
            fin_result += &html!({ child.render() });
        }

        let result = html!(
            <section class="section is-small">
                <div class="container">
                    <h1 class="title">"Hello actix webserver!"</h1>
                    <h2 class="subtitle">"actix-web + htmx + rusqlite = <3"</h2>
                </div>
            </section>
        );

        //result
        fin_result
    }
}

impl Children<T>
where
    T: Component,
{
    pub fn new() -> Self {
        Children { list: Vec::new() }
    }

    pub fn from_vec<T>(list: Vec<T>) -> Self {
        Children { list }
    }
}
