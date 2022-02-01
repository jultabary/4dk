use crate::domain::article::Article;
use dddk_macro::Event;
use dddk_core::dddk::event::event::Event;
use std::any::Any;

#[derive(Event, Debug)]
pub struct ArticleHasBeenPublishedEvent {
    pub title: String,
}

impl ArticleHasBeenPublishedEvent {
    pub fn new(article: &Article) -> ArticleHasBeenPublishedEvent {
        ArticleHasBeenPublishedEvent {
            title: article.get_title().clone(),
        }
    }
}
