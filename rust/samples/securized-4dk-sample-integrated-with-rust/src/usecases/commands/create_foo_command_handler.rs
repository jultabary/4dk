use std::any::{Any, TypeId};
use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use uuid::Uuid;
use crate::domain::foo::Foo;
use crate::domain::repository::FooRepository;
use crate::usecases::events::foo_created_event::FooCreatedEvent;

pub struct CreateFooCommand {
    id: Uuid,
    title: String,
}

impl CreateFooCommand {
    pub fn new(id: Uuid, title: String) -> CreateFooCommand {
        CreateFooCommand {
            id,
            title,
        }
    }
}

impl Command for CreateFooCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CreateFooCommandHandler {
    foo_repository: Rc<dyn FooRepository>,
}

impl CreateFooCommandHandler {
    pub fn new(foo_repository: Rc<dyn FooRepository>) -> CreateFooCommandHandler {
        CreateFooCommandHandler {
            foo_repository
        }
    }
}

impl CommandHandler<CreateFooCommand> for CreateFooCommandHandler {
    fn handle(&self, command: &CreateFooCommand) -> Events {
        let foo = Foo::new(command.id.clone(), command.title.clone());
        let foo_created_event = FooCreatedEvent::new(
            foo.get_id().clone(), foo.get_title().clone()
        );
        self.foo_repository.save(foo);
        let mut events = Vec::new() as Vec<Arc<dyn Event>>;
        events.push(Arc::new(foo_created_event));
        Ok(events)
    }
}

impl CommandHandlerInBus for CreateFooCommandHandler {
    fn handle_from_bus(&self, command: &dyn Command) -> Events {
        return self.handle_generic_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
