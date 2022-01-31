use std::rc::Rc;
use std::any::{Any, TypeId};
use dddk_core::dddk::aliases::ResponseFromHandler;
use dddk_core::dddk::query::response::Response;
use dddk_core::dddk::query::query::Query;
use dddk_core::dddk::query::query_handler::{QueryHandler, QueryHandlerInBus};
use dddk_macro::{Query, QueryHandlerInBus, Response};
use crate::domain::news_paper_repository::NewsPaperRepository;
use crate::domain::response::news_paper_response::NewsPaperResponse;

#[derive(Query, Debug)]
pub struct WhatAreOpenedNewsPaperQuery {}

#[derive(Response)]
pub struct NewsPapersResponse {
    news_papers: Vec<NewsPaperResponse>,
}

impl NewsPapersResponse {
    pub fn move_news_papers(&mut self) -> Vec<NewsPaperResponse> {
        std::mem::replace(&mut self.news_papers, Vec::new())
    }
}

#[derive(QueryHandlerInBus)]
pub struct WhatAreOpenedNewsPaperQueryHandler {
    news_paper_repository: Rc<dyn NewsPaperRepository>,
}

impl WhatAreOpenedNewsPaperQueryHandler {
    pub fn new(news_paper_repository: Rc<dyn NewsPaperRepository>) -> WhatAreOpenedNewsPaperQueryHandler {
        WhatAreOpenedNewsPaperQueryHandler {
            news_paper_repository
        }
    }
}

impl QueryHandler<WhatAreOpenedNewsPaperQuery> for WhatAreOpenedNewsPaperQueryHandler {
    fn handle(&self, _query: &WhatAreOpenedNewsPaperQuery) -> ResponseFromHandler {
        Ok(Box::new(NewsPapersResponse { news_papers: self.news_paper_repository.find_all() }))
    }
}