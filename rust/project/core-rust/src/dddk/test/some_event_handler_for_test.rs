use std::any::{Any, TypeId};
use crate::dddk::event::event::Event;
use crate::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
use crate::dddk::test::some_event_for_test::{AnEvent, AnotherEvent, EventForTest};

struct EventHandled {
    type_id: TypeId,
    event_id: i32,
}

impl EventHandled {
    fn new(type_id: TypeId, event_id: i32) -> EventHandled {
        EventHandled {
            type_id,
            event_id,
        }
    }
}

static mut EVENT_HANDLE_BY_AN_EVENT_HANDLER: Vec<EventHandled> = Vec::new();

pub fn reset_event_handled() {
    unsafe {
        EVENT_HANDLE_BY_AN_EVENT_HANDLER = Vec::new();
    }
}

pub unsafe fn has_event_been_handled_by_handler(event: &dyn EventForTest, event_handler_type_id: TypeId) -> bool {
    let event_opt = EVENT_HANDLE_BY_AN_EVENT_HANDLER
        .iter()
        .find(|event_handled| {
            event_handled.event_id == event.get_id() && event_handled.type_id == event_handler_type_id
        });
    let has_been_handled = event_opt.is_some();
    return has_been_handled;
}

pub struct AnEventHandler {}

impl AnEventHandler {
    pub fn new() -> AnEventHandler {
        AnEventHandler {}
    }
}

impl EventHandler<AnEvent> for AnEventHandler {
    fn handle(&self, event: &AnEvent) {
        unsafe {
            let an_event = event.as_any().downcast_ref::<AnEvent>().unwrap();
            EVENT_HANDLE_BY_AN_EVENT_HANDLER.push(
                EventHandled::new(
                    TypeId::of::<AnEventHandler>(),
                    an_event.get_id().clone())
            );
        }
    }
}

impl EventHandlerInBus for AnEventHandler {
    fn handle_from_bus<'a>(&self, event: &'a dyn Event) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.get_associated_event()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AnotherEventHandler {}

impl EventHandler<AnotherEvent> for AnotherEventHandler {
    fn handle(&self, event: &AnotherEvent) {
        unsafe {
            let another_event = event.as_any().downcast_ref::<AnotherEvent>().unwrap();

            EVENT_HANDLE_BY_AN_EVENT_HANDLER.push(
                EventHandled::new(
                    TypeId::of::<AnotherEventHandler>(),
                    another_event.get_id().clone())
            );
        }
    }
}

impl EventHandlerInBus for AnotherEventHandler {
    fn handle_from_bus<'a>(&self, event: &'a dyn Event) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.get_associated_event()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AnotherEventHandler {
    pub fn new() -> AnotherEventHandler {
        AnotherEventHandler {}
    }
}


pub struct AThirdEventHandler {}

impl EventHandler<AnEvent> for AThirdEventHandler {
    fn handle(&self, event: &AnEvent) {
        unsafe {
            let an_event = event.as_any().downcast_ref::<AnEvent>().unwrap();

            EVENT_HANDLE_BY_AN_EVENT_HANDLER.push(
                EventHandled::new(
                    TypeId::of::<AThirdEventHandler>(),
                    an_event.get_id().clone())
            );
        }
    }
}

impl EventHandlerInBus for AThirdEventHandler {
    fn handle_from_bus<'a>(&self, event: &'a dyn Event) {
        self.handle_generic_event(event);
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        self.get_associated_event()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AThirdEventHandler {
    pub fn new() -> AThirdEventHandler {
        AThirdEventHandler {}
    }
}
