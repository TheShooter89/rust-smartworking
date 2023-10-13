use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::{Component, StatisticsBox, StatisticsBoxData, StatisticsBoxDataGroup};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsManager {
    stats_groups: Vec<StatisticsBoxDataGroup>,
}

impl Component for StatisticsManager {
    fn render(&self) -> String {
        if self.stats_groups.is_empty() {
            return html!(<h1 class="subtitle box cw-statisticsmanager-nostats">"No stats to show yet"</h1>);
        }

        let mut groups_list = String::new();
        for group in &self.stats_groups {
            println!("group is: {:?}", group);
            let stat_box = StatisticsBox::new(&group.name);
            for stat in &group.data {
                groups_list = format!(
                    "{}{}",
                    groups_list,
                    StatisticsBox::new(&group.name)
                        .group(group.clone())
                        .render()
                )
            }
        }

        let result = html!(
            <section class="section is-small">
                <div class="container">
                    <h1 class="title">"Hello actix webserver!"</h1>
                    <h2 class="subtitle">"actix-web + htmx + rusqlite = <3"</h2>
                </div>
            </section>
        );

        result
    }
}

impl StatisticsManager {
    pub fn new() -> Self {
        StatisticsManager {
            stats_groups: Vec::new(),
        }
    }

    pub fn stats_group(&mut self, stats: StatisticsBoxDataGroup) {
        self.stats_groups.push(stats)
    }
}
