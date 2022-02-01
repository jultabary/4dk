use dddk_core::dddk::external_event::external_event::ExternalEvent;
use dddk_macro::ExternalEvent;
use std::any::Any;

#[derive(Debug, ExternalEvent)]
pub struct ArticleHasBeenReviewedEvent {
    pub news_paper_name: String,
    pub article_title: String,
    pub is_validated: bool
}