use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Head {
    title: String,
}

impl Component for Head {
    fn render(&self) -> String {
        let result = html!(
            <head>
                <title>{&self.title}</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link href="https://fonts.googleapis.com/css?family=Comfortaa" rel="stylesheet" />
                <link href="https://fonts.googleapis.com/css?family=Unica+One" rel="stylesheet" />
                <link href="static/bulma.min.css" rel="stylesheet" />
                <link href="static/css/font-awesome.min.css" rel="stylesheet" />
                <link href="static/custom_styles.css" rel="stylesheet" />
                <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous"></script>
            </head>
        );

        result
    }
}

impl Head {
    pub fn new(title: &str) -> Self {
        Head {
            title: title.to_string(),
        }
    }
}
