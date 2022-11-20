use std::sync::{Arc, Mutex};
use crate::usecases::events::new_article_submitted_event::NewArticleSubmittedEvent;
use serde::Serialize;
use kafka::producer::Producer;
use crate::domain::review_application_repository::ReviewApplicationRepository;
use crate::infrastructure::kafka::generic_producer::send_message;

#[derive(Serialize)]
pub struct ArticleSubmittedMessage {
    pub title: String,
}

pub struct ReviewApplicationKafkaProducer {
    kafka_producer: Arc<Mutex<Producer>>,
}

impl ReviewApplicationKafkaProducer {
    pub fn new(kafka_producer: Arc<Mutex<Producer>>) -> ReviewApplicationKafkaProducer {
        ReviewApplicationKafkaProducer {
            kafka_producer
        }
    }
}

impl ReviewApplicationRepository for ReviewApplicationKafkaProducer {
    fn broadcast_article_submitted_event(&self, article_submitted_event: &NewArticleSubmittedEvent) {
        let message = serde_json::to_string(&ArticleSubmittedMessage { title: article_submitted_event.title.clone() }).unwrap();
        send_message(self.kafka_producer.clone(), "article.submitted", message);
    }
}