#[cfg(test)]
mod tests {
    use crate::dddk::errors::QueryIsNotAssociatedWithHandler;
    use crate::dddk::query::query_handler::QueryHandler;
    use crate::dddk::test_tools::some_query_for_test::some_query_for_test::{AnotherQuery, AQuery};
    use crate::dddk::test_tools::some_query_handler_for_test::some_query_handler_for_test::AQueryHandler;
    use crate::dddk::test_tools::some_response_for_test::some_response_for_test::AResponse;

    #[test]
    fn it_should_handle_query_when_query_is_associated_to_this_handler() {
        // Given
        let a_query_handler = AQueryHandler::new();
        let a_query = AQuery { };

        // When
        let responses = a_query_handler.handle_generic_query(&a_query);

        // Then
        assert_eq!(true, responses.is_ok());
        let responses = responses.unwrap();
        assert_eq!(1, responses.len());
        let response = responses.get(0).unwrap();
        let a_response = response.as_ref().as_any().downcast_ref::<AResponse>();
        assert_eq!(true, a_response.is_some());
    }

    #[test]
    fn it_should_not_handle_query_when_query_is_not_associated_to_this_handler() {
        // Given
        let a_query_handler = AQueryHandler::new();
        let another_query = AnotherQuery { };

        // When
        let responses = a_query_handler.handle_generic_query(&another_query);

        // Then
        assert_eq!(true, responses.is_err());
        assert_eq!(true, responses.err().unwrap().downcast_ref::<QueryIsNotAssociatedWithHandler>().is_some());
    }
}