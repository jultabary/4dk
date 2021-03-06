pub mod commands {
    pub mod submit_article_command_handler;
    mod submit_article_command_handler_test;
    pub mod create_news_paper_command_handler;
    mod create_news_paper_command_handler_test;
    pub mod publish_article_command_handler;
    mod publish_article_command_handler_test;
}
pub mod queries {
    pub mod what_are_news_papers_query_handler;
    pub mod what_are_news_papers_query_handler_even_with_unpublished_articles;
    mod what_are_news_papers_query_handler_test;
    mod what_are_news_papers_query_handler_even_with_unpublished_articles_test;
}
pub mod events {
    pub mod news_paper_created_event;
    pub mod new_article_submitted_event;
    pub mod article_has_been_published_event;
}
pub mod policies {
    pub mod publish_article_if_validated_policy_handler;
    mod publish_article_if_validated_policy_handler_test;
    pub mod article_has_been_reviewed_event;
}
pub mod test {
    pub mod fake_news_paper_repository;
}