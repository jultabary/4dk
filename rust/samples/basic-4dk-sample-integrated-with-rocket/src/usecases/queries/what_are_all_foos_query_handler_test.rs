#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;
    use dddk_core::dddk::query::query_handler::QueryHandler;
    use dddk_core::dddk::query::response::Response;
    use uuid::Uuid;
    use crate::WhatAreAllTheFoosQueryHandler;
    use crate::domain::foo::Foo;
    use crate::domain::repository::FooRepository;
    use crate::usecases::queries::what_are_all_foos_query_handler::WhatAreAllTheFoosQuery;

    struct FooRepositoryFake {
    }
    impl FooRepository for FooRepositoryFake {
        fn get_all_foo(&self) -> Vec<Box<dyn Response>> {
            vec![
                Box::new(
                    Foo::new(Uuid::from_str("ee2c4426-4cb9-4671-8ef1-0e18d57bb2cd").unwrap(),
                             String::from("Hello World !!"))
                )
            ]
        }

        fn save(&self, _foo: Foo) {
            todo!()
        }
    }

    fn assert_expected_responses(responses: Vec<Box<dyn Response>>) {
        let foo = responses.get(0).unwrap().as_ref().as_any().downcast_ref::<Foo>().unwrap();
        assert_eq!(&Uuid::from_str("ee2c4426-4cb9-4671-8ef1-0e18d57bb2cd").unwrap(), foo.get_id());
    }

    fn assert_repo_has_been_called(responses: &Vec<Box<dyn Response>>) {
        assert_ne!(0, responses.len());
    }

    #[test]
    fn it_should_return_all_foos_persisted_in_repository_when_query_is_handled() {
        // Given
        let fake_repository = Arc::new(FooRepositoryFake {});
        let query_handler = WhatAreAllTheFoosQueryHandler::new(fake_repository);
        let query = WhatAreAllTheFoosQuery { };

        // When
        let responses = query_handler.handle_generic_query(&query);

        // Then
        assert_repo_has_been_called(&responses);
        assert_expected_responses(responses);
    }
}