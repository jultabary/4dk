use log::{error, info};
use crate::dddk::aliases::Responses;
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
    fn dispatch<'b>(&self, query: &'b dyn Query) -> Responses {
        info!("Dispatching a query [{}].", query.get_query_name());
        let responses = self.query_bus.dispatch(query);
        if responses.is_err() {
            let error = responses.err().unwrap();
            let error_message = error.to_string();
            error!(
                "An error has occurred when dispatching query [{}]: {}",
                query.get_query_name(),
                error_message
            );
            return Err(error);
        }
        let responses = responses.unwrap();
        let mut response_names = String::new();
        responses.iter()
            .for_each(|response| {
                response_names.push_str(response.get_response_name().as_str());
                response_names.push_str(" ");
            });
        info!(
            "Query[{}] has been handled and has produced [{}] responses [{}].",
            query.get_query_name(),
            responses.len(),
            response_names
        );
        return Ok(responses);
    }
}

