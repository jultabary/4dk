use std::rc::Rc;
use std::sync::Arc;
use dddk_core::dddk::command::bus_impl::command_dispatcher::CommandDispatcher;
use dddk_core::dddk::command::bus_impl::event_produced_by_command_bus_dispatcher::EventsProducedByCommandBusDispatcher;
use dddk_core::dddk::command::command_bus::CommandBus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::bus_impl::event_dispatcher::EventDispatcher;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use crate::domain::car::CarId;
use crate::domain::parking::Parking;
use crate::infrastructure::gate_repository::GateRepository;
use crate::infrastructure::parking_repository::ParkingRepository;
use crate::infrastructure::screen_repository::ScreenRepository;
use crate::usecases::command::park_car_command::{ParkCarCommand, ParkCarCommandHandler};
use crate::usecases::event::open_gate_policy::OpenGatePolicyHandler;
use crate::usecases::event::refresh_screen_policy::RefreshScreenPolicy;

pub mod domain;
pub mod infrastructure;
pub mod usecases;


fn main() {
    let a_parking = Parking::new(1, 10, Vec::new());
    let parking_repository = Rc::new(ParkingRepository::new(vec![a_parking]));
    let gate_repository = Rc::new(GateRepository::new());
    let screen_repository = Rc::new(ScreenRepository::new());

    let park_a_car_command_handler = ParkCarCommandHandler::new(parking_repository);
    let open_gate_policy = OpenGatePolicyHandler::new(gate_repository);
    let refresh_screen_policy = RefreshScreenPolicy::new(screen_repository);

    let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
    command_handlers.push(Box::new(park_a_car_command_handler));

    let mut event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;
    event_handlers.push(Box::new(open_gate_policy));
    event_handlers.push(Box::new(refresh_screen_policy));

    let command_dispatcher = CommandDispatcher::new(command_handlers);
    let event_dispatcher = EventDispatcher::new(event_handlers);

    let events_produced_by_command_bus_dispatcher = EventsProducedByCommandBusDispatcher::new(
        Box::new(command_dispatcher),
        Arc::new(event_dispatcher),
        false
    );

    let command = ParkCarCommand::new(CarId::new(1), 1);
    let _events = events_produced_by_command_bus_dispatcher.dispatch(&command);
}
