use std::rc::Rc;
use std::any::{Any, TypeId};
use dddk_core::dddk::aliases::ResponseFromHandler;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_handler::{QueryHandler, QueryHandlerInBus};
use dddk_macro::{Query, QueryHandlerInBus};
use crate::domain::response::news_paper_query_repository::NewsPaperQueryRepository;
use crate::domain::response::news_paper_response::NewsPapersResponse;

#[derive(Query, Debug)]
pub struct WhatAreNewsPaperEventWithUnpublishedArticlesQuery {}

#[derive(QueryHandlerInBus)]
pub struct WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler {
    news_paper_repository: Rc<dyn NewsPaperQueryRepository>,
}

impl WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler {
    pub fn new(news_paper_repository: Rc<dyn NewsPaperQueryRepository>) -> WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler {
        WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler {
            news_paper_repository
        }
    }
}

impl QueryHandler<WhatAreNewsPaperEventWithUnpublishedArticlesQuery> for WhatAreNewsPaperEventWithUnpublishedArticlesQueryHandler {
    fn handle(&self, _query: &WhatAreNewsPaperEventWithUnpublishedArticlesQuery) -> ResponseFromHandler {
        Ok(Box::new(NewsPapersResponse { news_papers: self.news_paper_repository.find_all_even_with_unpublished_article() }))
    }
}