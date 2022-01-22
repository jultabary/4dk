use std::any::{Any, TypeId};
use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::event::event::Event;
use dddk_core::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
use dddk_macro::EventHandlerInBus;
use crate::ScreenRepository;
use crate::usecases::event::new_car_is_registered_event::ANewCarIsParkedEvent;

#[derive(EventHandlerInBus)]
pub struct RefreshScreenPolicy {
    screen_repository: Rc<ScreenRepository>,
}

impl RefreshScreenPolicy {
    pub fn new(screen_repository: Rc<ScreenRepository>) -> RefreshScreenPolicy {
        RefreshScreenPolicy { screen_repository }
    }
}

impl EventHandler<ANewCarIsParkedEvent> for RefreshScreenPolicy {
    fn handle(&self, _event: &ANewCarIsParkedEvent) {
        self.screen_repository.refresh_screen();
    }
}
