#[cfg(test)]
mod what_are_news_papers_query_handler_even_with_unpublished_articles_test {
    use std::rc::Rc;
    use dddk_core::dddk::query::query_handler::QueryHandler;
    use crate::usecases::queries::what_are_news_papers_query_handler_even_with_unpublished_articles::{WhatAreNewsPaperEventWithUnpublishedArticlesQuery, WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler};
    use crate::usecases::test::fake_news_paper_repository::fake_repository::FakeNewsPaperQueryRepository;

    #[test]
    fn it_should_call_correct_repository_method() {
        // Given
        let repository = Rc::new(FakeNewsPaperQueryRepository::new());
        let query_handler = WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler::new(repository.clone());
        let query = WhatAreNewsPaperEventWithUnpublishedArticlesQuery {};

        // When
        let _response = query_handler.handle(&query);

        // Then
        assert_eq!(true, repository.get_find_all_with_unpublished_article().take());
    }

}