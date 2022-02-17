use dddk_macro::ExternalEvent;

#[derive(Debug, ExternalEvent)]
pub struct ArticleHasBeenReviewedEvent {
    pub news_paper_name: String,
    pub article_title: String,
    pub is_validated: bool
}