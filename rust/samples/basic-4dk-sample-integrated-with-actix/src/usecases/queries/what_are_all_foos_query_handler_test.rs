#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::str::FromStr;
    use dddk_core::dddk::query::query_handler::QueryHandler;
    use dddk_core::dddk::query::response::Response;
    use uuid::Uuid;
    use crate::WhatAreAllTheFoosQueryHandler;
    use crate::domain::foo::Foo;
    use crate::domain::repository::FooRepository;
    use crate::usecases::queries::foos_response::FoosResponse;
    use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQuery;

    struct FooRepositoryFake {}

    impl FooRepository for FooRepositoryFake {
        fn get_all_foo(&self) -> Box<dyn Response> {
            let foos = vec![
                Foo::new(Uuid::from_str("ee2c4426-4cb9-4671-8ef1-0e18d57bb2cd").unwrap(),
                         String::from("Hello World !!"))
            ];
            Box::new(FoosResponse::new(foos))
        }

        fn save(&self, _foo: Foo) {
            todo!()
        }
    }

    fn assert_expected_responses(response: Box<dyn Response>) {
        let foo = response.as_any().downcast_ref::<FoosResponse>().unwrap();
        assert_eq!(1, foo.get_foos().len());
        assert_eq!(&Uuid::from_str("ee2c4426-4cb9-4671-8ef1-0e18d57bb2cd").unwrap(), foo.get_foos().get(0).unwrap().get_id());
    }

    #[test]
    fn it_should_return_all_foos_persisted_in_repository_when_query_is_handled() {
        // Given
        let fake_repository = Rc::new(FooRepositoryFake {});
        let query_handler = WhatAreAllTheFoosQueryHandler::new(fake_repository);
        let query = WhatAreAllTheFoosQuery {};

        // When
        let response = query_handler.handle_generic_query(&query);

        // Then
        assert_eq!(true, response.is_ok());
        let response = response.unwrap();
        assert_expected_responses(response);
    }
}