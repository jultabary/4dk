use std::any::{Any, TypeId};
use std::sync::Arc;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use uuid::Uuid;
use crate::domain::foo::Foo;
use crate::domain::repository::FooRepository;

pub struct CreateFooCommand {
    id: Uuid,
    title: String
}
impl CreateFooCommand {
    pub fn new(id: Uuid, title: String) -> CreateFooCommand {
        CreateFooCommand {
            id,
            title
        }
    }
}
impl Command for CreateFooCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CreateFooCommandHandler {
    foo_repository: Arc<dyn FooRepository>,
}

impl CreateFooCommandHandler {
    pub fn new(foo_repository: Arc<dyn FooRepository>) -> CreateFooCommandHandler {
        CreateFooCommandHandler {
            foo_repository
        }
    }
}

impl CommandHandler<CreateFooCommand> for CreateFooCommandHandler {
    fn handle(&self, command: &CreateFooCommand) -> Vec<Box<dyn Event>> {
        let foo = Foo::new(command.id.clone(), command.title.clone());
        self.foo_repository.save(foo);
        return Vec::new();
    }
}

impl CommandHandlerInBus for CreateFooCommandHandler {
    fn handle_from_bus(&self, command: &dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_generic_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
