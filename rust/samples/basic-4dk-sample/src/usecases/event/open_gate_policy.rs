use std::rc::Rc;
use dddk_core::dddk::aliases::GenericError;
use dddk_core::dddk::event::event_handler::EventHandler;
use dddk_macro::EventHandlerInBus;
use crate::infrastructure::gate_repository::GateRepository;
use crate::usecases::event::new_car_is_registered_event::ANewCarIsParkedEvent;

#[derive(EventHandlerInBus)]
pub struct OpenGatePolicyHandler {
    gate_repository: Rc<GateRepository>,
}

impl OpenGatePolicyHandler {
    pub fn new(gate_repository: Rc<GateRepository>) -> OpenGatePolicyHandler {
        OpenGatePolicyHandler { gate_repository }
    }
}

impl EventHandler<ANewCarIsParkedEvent> for OpenGatePolicyHandler {
    fn handle(&self, _event: &ANewCarIsParkedEvent) -> Result<(), GenericError> {
        self.gate_repository.open_gate();
        Ok(())
    }
}
