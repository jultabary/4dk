use crate::domain::article::Article;
use dddk_macro::Event;

#[derive(Event, Debug)]
pub struct ArticleHasBeenPublishedEvent {
    pub title: String,
}

impl ArticleHasBeenPublishedEvent {
    pub fn new(article: &Article) -> ArticleHasBeenPublishedEvent {
        ArticleHasBeenPublishedEvent {
            title: article.get_title().clone(),
        }
    }
}
