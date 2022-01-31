use chrono::{DateTime, Utc};
use crate::domain::article::Article;
use dddk_macro::Event;
use dddk_core::dddk::event::event::Event;
use std::any::Any;

#[derive(Event, Debug)]
pub struct ArticleHasBeenPublishedEvent {
    title: String,
    date_time: DateTime<Utc>,
}

impl ArticleHasBeenPublishedEvent {
    pub fn new(article: &Article) -> ArticleHasBeenPublishedEvent {
        ArticleHasBeenPublishedEvent {
            title: article.get_title().clone(),
            date_time: Utc::now(),
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_date_time(&self) -> &DateTime<Utc> {
        &self.date_time
    }
}
