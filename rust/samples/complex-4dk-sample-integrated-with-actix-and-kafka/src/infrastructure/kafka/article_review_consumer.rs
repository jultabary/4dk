use std::sync::Arc;
use log::info;
use serde::Deserialize;
use crate::Context;
use crate::usecases::policies::article_has_been_reviewed_event::ArticleHasBeenReviewedEvent;

pub fn consume_article_review_event(message: &str, context: &Arc<Context>) {
    let article_review = serde_json::from_str::<ArticleReview>(message).unwrap();
    info!("Received message: {:?}", article_review);
    let external_event = ArticleHasBeenReviewedEvent {
        news_paper_name: article_review.news_paper_name.clone(),
        article_title: article_review.article_title.clone(),
        is_validated: article_review.is_validate
    };
    let _commands = context.bus.dispatch_external_event(&external_event);
}

#[derive(Deserialize, Debug)]
pub struct ArticleReview {
    pub news_paper_name: String,
    pub article_title: String,
    pub is_validate: bool
}