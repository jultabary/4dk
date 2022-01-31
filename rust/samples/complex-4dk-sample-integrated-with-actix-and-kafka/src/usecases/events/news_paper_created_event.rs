use chrono::{DateTime, Utc};
use dddk_macro::Event;
use dddk_core::dddk::event::event::Event;
use std::any::Any;

#[derive(Event, Debug)]
pub struct NewsPaperCreatedEvent {
    name: String,
    date_time: DateTime<Utc>,
}

impl NewsPaperCreatedEvent {
    pub fn new(news_paper_name: &String) -> NewsPaperCreatedEvent {
        NewsPaperCreatedEvent {
            name: news_paper_name.clone(),
            date_time: Utc::now(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_date_time(&self) -> &DateTime<Utc> {
        &self.date_time
    }
}