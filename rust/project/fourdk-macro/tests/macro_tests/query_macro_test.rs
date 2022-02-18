#[cfg(test)]
pub mod query_macro_tests {
    extern crate dddk_macro;
    use std::any::{Any, TypeId};
    use std::fmt::{Debug, Formatter};
    use dddk_core::dddk::aliases::ResponseFromHandler;
    use dddk_core::dddk::query::query_handler::QueryHandler;
    use dddk_core::dddk::query::query::Query;
    use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
    use dddk_macro::Query;
    use dddk_macro::QueryHandlerInBus;
    use dddk_macro::Response;

    #[derive(Query)]
    struct AQuery {}

    impl Debug for AQuery {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AQuery")
        }
    }

    #[derive(Response)]
    struct AResponse {}

    #[derive(QueryHandlerInBus)]
    struct AQueryHandler {}

    impl QueryHandler<AQuery> for AQueryHandler {
        fn handle(&self, _query: &AQuery) -> ResponseFromHandler {
            Ok(Box::new(AResponse {}))
        }
    }

    #[test]
    fn it_should_impl_default_behaviour_of_query_trait_when_using_derive_macro() {
        // Given
        let a_query = AQuery {};

        // When
        let query_name = a_query.get_query_name();
        let as_any: &dyn Any = a_query.as_any();

        // Then
        assert_eq!("AQuery".to_string(), query_name);
        assert_eq!(true, as_any.downcast_ref::<AQuery>().is_some());
    }

    #[test]
    fn it_should_impl_default_behaviour_of_query_handler_in_bus_trait_when_using_derive_macro() {
        // Given
        let a_query = AQuery {};
        let a_query_handler = AQueryHandler {};

        // When
        let response: ResponseFromHandler = a_query_handler.handle_from_bus(&a_query);
        let query_handler_name = a_query_handler.get_query_handler_name();
        let query_type_id = a_query_handler.get_associated_query_from_bus();

        // Then
        let mut response = response.unwrap();
        assert_eq!(true, response.as_any().downcast_ref::<AResponse>().is_some());
        assert_eq!("AQueryHandler".to_string(), query_handler_name);
        assert_eq!(TypeId::of::<AQuery>(), query_type_id);

    }
}