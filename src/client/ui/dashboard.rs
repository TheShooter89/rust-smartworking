use std::collections::HashMap;

use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use html_to_string_macro::html;

use crate::client::ui::Layout::Children;
use crate::client::ui::{
    ChatPanel, Component, StatisticsBox, StatisticsBoxData, StatisticsBoxDataGroup,
    StatisticsManager, TitleStrip,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {}

impl Component for Dashboard {
    fn render(&self) -> String {
        let mut stat_box = StatisticsBoxDataGroup::new();
        stat_box.name("Aggregate");

        stat_box.data.push(StatisticsBoxData {
            label: "Total days:".to_string(),
            value: "217".to_string(),
        });
        stat_box.data.push(StatisticsBoxData {
            label: "Total messages (raw):".to_string(),
            value: "18673(16736)".to_string(),
        });
        stat_box.data.push(StatisticsBoxData {
            label: "From me:".to_string(),
            value: "6718(5972)".to_string(),
        });
        println!("stat_box: {:?}", stat_box);

        let mut another_box = StatisticsBoxDataGroup::new();
        another_box.data.push(StatisticsBoxData {
            label: "Average message/day:".to_string(),
            value: "12.7(9.23)".to_string(),
        });
        another_box.data.push(StatisticsBoxData {
            label: "MAX messages in a single day:".to_string(),
            value: "37(22)".to_string(),
        });
        another_box.data.push(StatisticsBoxData {
            label: "MIN messages in a single day:".to_string(),
            value: "1(0.75)".to_string(),
        });

        // TESTING
        //let credits = TitleStrip::new().add("prova");
        //let fake_stats = ChatPanel::new();
        //let mut children = Vec::new();
        //children.push(credits.clone());
        //children.push(fake_stats);
        //let children_render = Children::from_vec(Vec::new().push(credits).push(fake_stats));

        let result = html!(
            <section class="section p-4 cw-dashboard-container">
                <section class="columns">
                    <div class="column is-one-fifth">
                        {ChatPanel::new().render()}
                    </div>

                    <div class="column">
                        {TitleStrip::new().add("Andrea").add("Carla").render()}
                    </div>

                    <div class="column is-one-quarter">
                        {TitleStrip::new().add("statistics").render()}
                        {StatisticsBox::new("Totals").group(stat_box).group(another_box).render()}
                        {StatisticsManager::new().render()}
                    </div>
                </section>
            </section>
        );

        result
    }
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {}
    }
}
