use std::any::{Any, TypeId};
use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::event::event::Event;
use dddk_core::dddk::event::event_handler::{EventHandler, EventHandlerInBus};
use crate::infrastructure::gate_repository::GateRepository;
use crate::usecases::event::new_car_is_registered_event::ANewCarIsParkedEvent;

pub struct OpenGatePolicyHandler {
    gate_repository: Rc<GateRepository>,
}

impl OpenGatePolicyHandler {
    pub fn new(gate_repository: Rc<GateRepository>) -> OpenGatePolicyHandler {
        OpenGatePolicyHandler { gate_repository }
    }
}

impl EventHandler<ANewCarIsParkedEvent> for OpenGatePolicyHandler {
    fn handle(&self, _event: &ANewCarIsParkedEvent) {
        self.gate_repository.open_gate();
    }
}

impl EventHandlerInBus for OpenGatePolicyHandler {
    fn handle_from_bus(&self, event: Arc<dyn Event>) {
        self.handle_generic_event(event)
    }

    fn get_associated_event_from_bus(&self) -> TypeId {
        TypeId::of::<ANewCarIsParkedEvent>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}