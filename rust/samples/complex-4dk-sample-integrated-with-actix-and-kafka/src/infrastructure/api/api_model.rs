use ::serde::Deserialize;

#[derive(Deserialize)]
pub struct NewsPaperBodyRequest {
    pub name: String,
}