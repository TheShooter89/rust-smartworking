use std::result;

use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

fn login_card() -> String {
    let result = html!(
        <article class="box">
            <section class="level">
                <section class="level-item">
                    <figure class="image is-96x96">
                        <img src="static/img/rsw_logo.png" alt="rsw logo" />
                    </figure>
                </section>
            </section>

            <section class="level">
                <section class="level-item">
                    <h1 class="title">
                        "rust-smartworking"
                    </h1>
                </section>
            </section>

            <section class="level">
                <section class="level-item">
                    <h2 class="subtitle">
                        "AirFreight Operations online manager"
                    </h2>
                </section>
            </section>

            <section class="level rsw-loginpage-formbox-quote">
                <section class="level-item">
                    <q class="is-italic notification is-danger">
                        "War is "<span class="has-text-weight-bold">"HARD"</span>", but "<span class="has-text-weight-bold">"LOGISTICS"</span>" is "<span class="has-text-weight-bold">"HARDER"</span>""
                    </q>
                </section>
            </section>

            <section class="is-fullwidth">
                <form class="">
                    <div class="field">
                        <label class="label">"Login using your email and a magic link"</label>
                        <div class="control">
                            <input class="input" type="text" placeholder="your.email@provider.ua" />
                        </div>
                    </div>

                    <div class="field">
                        <div class="control">
                            <button hx-post="/send_magic_link" class="button is-warning is-fullwidth">"SEND MAGIC LINK"</button>
                        </div>
                    </div>
                </form>
            </section>
        </article>
    );

    result
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPage {}

impl Component for LoginPage {
    fn render(&self) -> String {
        let result = html!(
            <section class="section is-small rsw-loginpage-section rsw-loginpage-section-background">
                <div class="columns">
                    <div class="column is-3 is-offset-7">
                        {login_card()}
                    </div>
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
