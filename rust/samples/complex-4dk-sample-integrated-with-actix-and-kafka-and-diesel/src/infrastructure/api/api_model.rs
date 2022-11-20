use ::serde::Deserialize;

#[derive(Deserialize)]
pub struct NewsPaperBodyRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SubmitArticleRequest {
    pub title: String,
    pub body: String,
}