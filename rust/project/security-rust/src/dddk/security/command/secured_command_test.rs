#[cfg(test)]
mod test {
    use dddk_core::dddk::command::command::Command;
    use crate::dddk::security::command::secured_command::SecuredCommand;
    use crate::dddk::security::test_tools::some_role_and_permission_for_test::some_role_and_permission_for_test::get_a_role;
    use crate::dddk::security::test_tools::some_secured_command_for_test::some_secured_command_for_test::{ACommand, get_a_command_secured};

    #[test]
    fn it_should_return_sub_command_when_call_get_command() {
        // Given
        let user_roles = vec![get_a_role().get_name().clone()];
        let a_command_secured = get_a_command_secured(user_roles);

        // When
        let command= a_command_secured.get_command();
        let a_command_opt = command.as_any().downcast_ref::<ACommand>();

        // Then
        assert_eq!(true, a_command_opt.is_some());
        assert_eq!(&vec![get_a_role().get_name().clone()], a_command_secured.get_roles_names());
    }

    #[test]
    fn it_should_downcast_ref_secured_command_when_receiving_it_as_command() {
        // Given
        let user_roles = vec![get_a_role().get_name().clone()];
        let a_command_secured = get_a_command_secured(user_roles);
        let command = &a_command_secured as &dyn Command;

        // When
        let a_secured_command_opt = command.as_any().downcast_ref::<SecuredCommand>();

        // Then
        assert_eq!(true, a_secured_command_opt.is_some());
        let a_command_secured = a_secured_command_opt.unwrap();
        let a_command_opt = a_command_secured.get_command().as_any().downcast_ref::<ACommand>();
        assert_eq!(true, a_command_opt.is_some());
    }

}