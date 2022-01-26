use log::{error, info};
use crate::dddk::aliases::ResponseFromHandler;
use crate::dddk::query::query::Query;
use crate::dddk::query::query_bus::QueryBus;

pub struct QueryLoggingMiddleware {
    query_bus: Box<dyn QueryBus>,
}

impl QueryLoggingMiddleware {
    pub fn new(query_bus: Box<dyn QueryBus>) -> QueryLoggingMiddleware {
        QueryLoggingMiddleware {
            query_bus
        }
    }
}

impl QueryBus for QueryLoggingMiddleware {
    fn dispatch<'b>(&self, query: &'b dyn Query) -> ResponseFromHandler {
        info!("Dispatching a query [{}] [{:?}].",
              query.get_query_name(),
              query);
        let response = self.query_bus.dispatch(query);
        if response.is_err() {
            let error = response.err().unwrap();
            let error_message = error.to_string();
            error!(
                "An error has occurred when dispatching query [{}] [{:?}]: {}",
                query.get_query_name(),
                query,
                error_message
            );
            return Err(error);
        }
        let response = response.unwrap();
        info!(
            "Query[{}] [{:?}] has been handled and has returned response [{}].",
            query.get_query_name(),
            query,
            response.get_response_name()
        );
        return Ok(response);
    }
}

