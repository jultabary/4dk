use std::sync::Arc;
use dddk_core::dddk::event::event::Event;
use crate::domain::article::Article;
use crate::domain::error::ArticleIsAlreadyPublished;
use crate::usecases::events::new_article_published_event::NewArticlePublishedEvent;
use crate::usecases::events::new_news_paper_opened_event::NewNewsPaperOpenedEvent;

pub struct NewsPaper {
    name: String,
    articles: Vec<Article>,
    generated_events: Vec<Arc<dyn Event>>,
}

impl NewsPaper {
    pub fn new(name: String) -> NewsPaper {
        let mut events = Vec::new() as Vec<Arc<dyn Event>>;
        events.push(Arc::new(NewNewsPaperOpenedEvent::new(&name.clone())));
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

    pub fn publish(&mut self, article: Article) -> Result<(), ArticleIsAlreadyPublished> {
        if let Some(_article) = self.get_article_by_name(article.get_title().clone()) {
            return Err(ArticleIsAlreadyPublished { article: article.get_title().clone() });
        } else {
            self.generated_events.push(Arc::new(NewArticlePublishedEvent::new(&article)));
            Ok(self.articles.push(article))
        }
    }

    pub fn get_article_by_name(&self, name: String) -> Option<&Article> {
        self.articles.iter()
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