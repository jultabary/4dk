use crate::domain::error::ArticleIsAlreadyPublished;
use crate::usecases::events::article_has_been_published_event::ArticleHasBeenPublishedEvent;

pub struct Article {
    title: String,
    body: String,
    is_published: bool,
}

impl Article {
    pub fn new(title: String, body: String) -> Article {
        Article {
            title,
            body,
            is_published: false,
        }
    }

    pub fn publish(&mut self) -> Result<ArticleHasBeenPublishedEvent, ArticleIsAlreadyPublished> {
        if self.is_published {
            Err(ArticleIsAlreadyPublished { article: self.title.clone() })
        } else {
            self.is_published = true;
            Ok(ArticleHasBeenPublishedEvent::new(self))
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_body(&self) -> &String {
        &self.body
    }
}