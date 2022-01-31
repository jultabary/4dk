use serde::Serialize;
use dddk_macro::Response;
use std::any::Any;
use dddk_core::dddk::query::response::Response;

#[derive(Serialize)]
pub struct NewsPaperResponse {
    name: String,
    articles: Vec<ArticleResponse>,
}

impl NewsPaperResponse {
    pub fn new(name: String, articles: Vec<ArticleResponse>) -> NewsPaperResponse {
        NewsPaperResponse {
            name,
            articles,
        }
    }
}

#[derive(Serialize)]
pub struct ArticleResponse {
    title: String,
    body: String,
    is_published: bool
}

impl ArticleResponse {
    pub fn new(title: String, body: String, is_published: bool) -> ArticleResponse {
        ArticleResponse {
            title,
            body,
            is_published
        }
    }
}

#[derive(Response)]
pub struct NewsPapersResponse {
    pub news_papers: Vec<NewsPaperResponse>,
}

impl NewsPapersResponse {
    pub fn move_news_papers(&mut self) -> Vec<NewsPaperResponse> {
        std::mem::replace(&mut self.news_papers, Vec::new())
    }
}
