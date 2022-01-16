#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use dddk_core::dddk::command::command_handler::CommandHandler;
    use dddk_core::dddk::query::response::Response;
    use uuid::Uuid;
    use crate::CreateFooCommandHandler;
    use crate::domain::foo::Foo;
    use crate::domain::repository::FooRepository;
    use crate::usecases::commands::create_foo_command_handler::CreateFooCommand;

    static mut SAVE_HAS_BEEN_CALLED: bool = false;

    struct FooRepositoryFake {}

    impl FooRepository for FooRepositoryFake {
        fn get_all_foo(&self) -> Vec<Box<dyn Response>> {
            vec![]
        }

        fn save(&self, _foo: Foo) {
            unsafe {
                SAVE_HAS_BEEN_CALLED = true;
            }
        }
    }

    fn assert_save_method_has_been_called() {
        unsafe {
            assert_eq!(true, SAVE_HAS_BEEN_CALLED);
        }
    }

    #[test]
    fn it_should_call_repository_when_command_is_handled() {
        // Given
        let fake_repository = Arc::new(FooRepositoryFake {});
        let command_handler = CreateFooCommandHandler::new(fake_repository);
        let command = CreateFooCommand::new(Uuid::new_v4(), String::from("Hello World !!"));

        // When
        command_handler.handle_generic_command(&command);

        // Then
        assert_save_method_has_been_called()
    }
}