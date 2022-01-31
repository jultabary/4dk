use std::sync::Arc;
use dddk_core::dddk::aliases::GenericError;
use dddk_core::dddk::event::event::Event;
use crate::domain::article::Article;
use crate::domain::error::{ArticleDoesNotExist, ArticleIsAlreadySubmitted};
use crate::usecases::events::new_article_published_event::NewArticleSubmittedEvent;
use crate::usecases::events::news_paper_created_event::NewsPaperCreatedEvent;

pub struct NewsPaper {
    name: String,
    articles: Vec<Article>,
    generated_events: Vec<Arc<dyn Event>>,
}

impl NewsPaper {
    pub fn new(name: String) -> NewsPaper {
        let mut events = Vec::new() as Vec<Arc<dyn Event>>;
        events.push(Arc::new(NewsPaperCreatedEvent::new(&name.clone())));
        NewsPaper {
            name,
            articles: Vec::new(),
            generated_events: events,
        }
    }

    pub fn reconstitute(name: String, articles: Vec<Article>) -> NewsPaper {
        NewsPaper {
            name,
            articles,
            generated_events: Vec::new(),
        }
    }

    pub fn submit(&mut self, article: Article) -> Result<(), ArticleIsAlreadySubmitted> {
        if let Some(_) = self.get_article_by_name(article.get_title().clone()) {
            return Err(ArticleIsAlreadySubmitted { article: article.get_title().clone() });
        } else {
            self.generated_events.push(Arc::new(NewArticleSubmittedEvent::new(&article)));
            Ok(self.articles.push(article))
        }
    }

    pub fn publish_article(&mut self, article_name: String) -> Result<(), GenericError> {
        return if let Some(article) = self.get_article_by_name(article_name.clone()) {
            match article.publish() {
                Ok(event) => {
                    self.generated_events.push(Arc::new(event));
                    Ok(())
                }
                Err(error) => {
                    Err(Box::new(error))
                }
            }
        } else {
            Err(Box::new(ArticleDoesNotExist { article: article_name.clone() }))
        }

    }

    pub fn get_article_by_name(&mut self, name: String) -> Option<&mut Article> {
        self.articles.iter_mut()
            .find(|published_article| { published_article.get_title().clone() == name })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_articles(&self) -> &Vec<Article> {
        &self.articles
    }


    pub fn get_generated_events(&self) -> &Vec<Arc<dyn Event>> {
        &self.generated_events
    }

    pub fn move_generated_events(&mut self) -> Vec<Arc<dyn Event>> {
        std::mem::replace(&mut self.generated_events, Vec::new())
    }
}