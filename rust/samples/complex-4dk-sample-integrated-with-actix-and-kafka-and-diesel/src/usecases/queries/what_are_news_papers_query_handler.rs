use std::rc::Rc;
use dddk_core::dddk::aliases::ResponseFromHandler;
use dddk_core::dddk::query::query_handler::QueryHandler;
use dddk_macro::{Query, QueryHandlerInBus};
use crate::domain::response::news_paper_query_repository::NewsPaperQueryRepository;
use crate::domain::response::news_paper_response::NewsPapersResponse;

#[derive(Query, Debug)]
pub struct WhatAreNewsPaperQuery {}

#[derive(QueryHandlerInBus)]
pub struct WhatAreNewsPaperQueryHandler {
    news_paper_repository: Rc<dyn NewsPaperQueryRepository>,
}

impl WhatAreNewsPaperQueryHandler {
    pub fn new(news_paper_repository: Rc<dyn NewsPaperQueryRepository>) -> WhatAreNewsPaperQueryHandler {
        WhatAreNewsPaperQueryHandler {
            news_paper_repository
        }
    }
}

impl QueryHandler<WhatAreNewsPaperQuery> for WhatAreNewsPaperQueryHandler {
    fn handle(&self, _query: &WhatAreNewsPaperQuery) -> ResponseFromHandler {
        Ok(Box::new(NewsPapersResponse { news_papers: self.news_paper_repository.find_all() }))
    }
}