use serde::Serialize;

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
}

impl ArticleResponse {
    pub fn new(title: String, body: String) -> ArticleResponse {
        ArticleResponse {
            title,
            body
        }
    }
}