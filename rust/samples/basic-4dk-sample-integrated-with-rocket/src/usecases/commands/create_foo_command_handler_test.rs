#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use dddk_core::dddk::query::response::Response;
    use uuid::Uuid;
    use crate::CreateFooCommandHandler;
    use crate::domain::foo::Foo;
    use crate::domain::repository::FooRepository;
    use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;
    use crate::usecases::queries::foos_response::FoosResponse;

    struct FooRepositoryFake {
        pub save_has_been_called: RefCell<Option<bool>>,
    }

    impl FooRepository for FooRepositoryFake {
        fn get_all_foo(&self) -> Box<dyn Response> {
            Box::new(FoosResponse::new(vec![]))
        }

        fn save(&self, _foo: Foo) {
            let _value = self.save_has_been_called.borrow_mut().insert(true);
        }
    }

    #[test]
    fn it_should_call_repository_when_command_is_handled() {
        // Given
        let fake_repository = Rc::new(FooRepositoryFake { save_has_been_called: RefCell::new(None) });
        let command_handler = CreateFooCommandHandler::new(fake_repository.clone());
        let command = CreateFooCommand::new(Uuid::new_v4(), String::from("Hello World !!"));

        // When
        let events = command_handler.handle_generic_command(&command);

        // Then
        assert_eq!(true, events.is_ok());
        assert_eq!(true, fake_repository.save_has_been_called.borrow().unwrap());
    }
}