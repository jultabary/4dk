use dddk_macro::Event;
use dddk_core::dddk::aliases::GenericError;
use dddk_core::dddk::event::event_handler::EventHandler;
use dddk_macro::EventHandlerInBus;
use crate::domain::article::Article;
use crate::domain::review_application_repository::ReviewApplicationRepository;

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

#[derive(EventHandlerInBus)]
pub struct BroadcastArticledSubmittedEventHandler {
    article_review_repository: Box<dyn ReviewApplicationRepository>
}

impl BroadcastArticledSubmittedEventHandler {
    pub fn new(article_review_repository: Box<dyn ReviewApplicationRepository>) -> BroadcastArticledSubmittedEventHandler {
        BroadcastArticledSubmittedEventHandler{
            article_review_repository
        }
    }
}

impl EventHandler<NewArticleSubmittedEvent> for BroadcastArticledSubmittedEventHandler {
    fn handle(&self, event: &NewArticleSubmittedEvent) -> Result<(), GenericError> {
        self.article_review_repository.broadcast_article_submitted_event(event);
        Ok(())
    }
}