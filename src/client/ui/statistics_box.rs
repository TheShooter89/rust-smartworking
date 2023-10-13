use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsBox {
    name: String,
    data_groups: Vec<StatisticsBoxDataGroup>,
}

impl Component for StatisticsBox {
    fn render(&self) -> String {
        let groups_tables_list: String = match &self.data_groups.len() {
            0 => html!(<h1 class="subtitle">"no statistics..."</h1>),
            _ => {
                let mut result = String::new();
                for group in &self.data_groups {
                    let mut rows = String::new();
                    for data_item in &group.data {
                        let label_td = html!(<td>{data_item.label.clone()}</td>);
                        let value_td = html!(<td><span class="has-text-weight-bold pr-1">{data_item.value.clone()}</span></td>);

                        let current_row = html!(<tr>{label_td}{value_td}</tr>);
                        rows = format!("{}{}", rows, current_row)
                    }

                    let current_group_table = html!(
                        <h1 class="subtitle">{group.name.clone()}</h1>
                        <table class="table is-fullwidth">
                            <tbody>
                                {rows}
                            </tbody>
                        </table>
                    );

                    result = format!("{}{}", result, current_group_table)
                }
                result
            }
        };

        let result = html!(
            <article class="message">
                <header class="message-header">
                    <p class="">{self.name.clone()}</p>
                    <button class="delete" aria-label="delete"></button>
                </header>
                <section class="message-body">
                    {groups_tables_list}
                </section>
            </article>
        );

        result
    }
}

impl StatisticsBox {
    pub fn new(name: &str) -> Self {
        StatisticsBox {
            name: name.to_string(),
            data_groups: Vec::new(),
        }
    }

    pub fn group(&mut self, new_group: StatisticsBoxDataGroup) -> &mut Self {
        self.data_groups.push(new_group);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsBoxDataGroup {
    pub name: String,
    pub data: Vec<StatisticsBoxData>,
}

impl StatisticsBoxDataGroup {
    pub fn new() -> Self {
        StatisticsBoxDataGroup {
            name: "<br />".to_string(),
            data: Vec::new(),
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // helper function to add a new StatisticsBoxData without directly instantiating it
    pub fn add_data(&mut self, label: &str, value: &str) {
        self.data.push(StatisticsBoxData {
            label: label.to_string(),
            value: value.to_string(),
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsBoxData {
    pub label: String,
    pub value: String,
}
