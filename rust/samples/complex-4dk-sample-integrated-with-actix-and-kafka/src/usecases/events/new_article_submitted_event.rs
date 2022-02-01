use dddk_macro::Event;
use dddk_core::dddk::event::event::Event;
use std::any::Any;
use crate::domain::article::Article;

#[derive(Event, Debug)]
pub struct NewArticleSubmittedEvent {
    pub title: String,
}

impl NewArticleSubmittedEvent {
    pub fn new(article: &Article) -> NewArticleSubmittedEvent {
        NewArticleSubmittedEvent {
            title: article.get_title().clone(),
        }
    }
}
