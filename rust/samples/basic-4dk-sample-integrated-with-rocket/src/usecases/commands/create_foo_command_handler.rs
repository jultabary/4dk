use std::any::{Any, TypeId};
use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use dddk_macro::Command;
use dddk_macro::CommandHandlerInBus;
use uuid::Uuid;
use crate::domain::foo::Foo;
use crate::domain::repository::FooRepository;
use crate::usecases::events::foo_created_event::FooCreatedEvent;

#[derive(Command)]
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

#[derive(CommandHandlerInBus)]
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
