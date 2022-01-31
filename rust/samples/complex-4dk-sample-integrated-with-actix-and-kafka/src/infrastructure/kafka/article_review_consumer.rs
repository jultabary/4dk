use std::sync::Arc;
use log::info;
use serde::Deserialize;
use crate::Context;

pub fn consume_article_review_event(message: &str, _context: &Arc<Context>) {
    let article_review = serde_json::from_str::<ArticleReview>(message).unwrap();
    info!("Received message: {:?}", article_review);

}

#[derive(Deserialize, Debug)]
pub struct ArticleReview {
    pub news_paper_name: String,
    pub article_title: String,
    pub is_validate: bool
}