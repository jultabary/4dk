pub mod commands {
    pub mod publish_article_command_handler;
    pub mod open_news_paper_command_handler;
}
pub mod queries {
    pub mod what_are_opened_news_papers_query_handler;
}
pub mod events {
    pub mod new_news_paper_opened_event;
    pub mod new_article_published_event;
}