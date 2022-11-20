use crate::usecases::events::new_article_submitted_event::NewArticleSubmittedEvent;

pub trait ReviewApplicationRepository {
    fn broadcast_article_submitted_event(&self, article_submitted_event: &NewArticleSubmittedEvent);
}