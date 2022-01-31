pub mod database {
    pub mod database_model;
    pub mod database_repository;
}
pub mod kafka {
    pub mod generic_consumer;
    pub mod config;
    pub mod article_review_consumer;
}
pub mod api {
    pub mod routes;
    pub mod error_handling;
    pub mod api_model;
}