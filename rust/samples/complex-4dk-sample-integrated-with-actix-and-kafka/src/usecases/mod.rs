pub mod commands {
    pub mod submit_article_command_handler;
    pub mod open_news_paper_command_handler;
    mod open_news_paper_command_handler_test;
    pub mod publish_article_command_handler;
}
pub mod queries {
    pub mod what_are_opened_news_papers_query_handler;
}
pub mod events {
    pub mod new_news_paper_opened_event;
    pub mod new_article_published_event;
    pub mod article_has_been_published_event;
}
pub mod policies {
    pub mod publish_article_if_validated_policy_handler;
    pub mod article_has_been_reviewed_event;
}