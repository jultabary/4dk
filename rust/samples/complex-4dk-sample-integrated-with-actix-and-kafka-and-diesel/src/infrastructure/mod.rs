pub mod database {
    pub mod database_model;
    pub mod database_repository;
    mod database_repository_integration_test;
    pub mod common_database_integration_test;
    pub mod database_query_repository;
}
pub mod kafka {
    pub mod generic_consumer;
    pub mod generic_producer;
    pub mod config;
    pub mod article_reviewed_consumer;
    pub mod article_submitted_producer;
}
pub mod api {
    pub mod routes;
    pub mod error_handling;
    pub mod api_model;
}