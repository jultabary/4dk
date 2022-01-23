use std::rc::Rc;
use std::thread;
use std::time::Duration;
use dddk_core::Bus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use log::LevelFilter;
use crate::domain::car::CarId;
use crate::domain::parking::Parking;
use crate::infrastructure::gate_repository::GateRepository;
use crate::infrastructure::parking_repository::ParkingRepository;
use crate::infrastructure::screen_repository::ScreenRepository;
use crate::simple_logger::SimpleLogger;
use crate::usecases::command::park_car_command::{ParkCarCommand, ParkCarCommandHandler};
use crate::usecases::event::open_gate_policy::OpenGatePolicyHandler;
use crate::usecases::event::refresh_screen_policy::RefreshScreenPolicy;

pub mod domain;
pub mod infrastructure;
pub mod usecases;
mod simple_logger;

static LOGGER: SimpleLogger = SimpleLogger {};

fn main() {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

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

    let bus = Bus::new(command_handlers, event_handlers, Vec::new());

    let command = ParkCarCommand::new(CarId::new(1), 1);
    let _events = bus.dispatch_command(&command);

    // Wait for all event_handlers handle theirs events
    thread::sleep(Duration::from_secs(2));
}
