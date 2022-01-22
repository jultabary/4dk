use std::any::{Any, TypeId};
use std::rc::Rc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandler, CommandHandlerInBus};
use dddk_macro::CommandHandlerInBus;
use dddk_macro::Command;
use crate::domain::car::CarId;
use crate::infrastructure::parking_repository::ParkingRepository;

#[derive(Command)]
pub struct ParkCarCommand {
    car_id: CarId,
    parking_id: i32,
}

impl ParkCarCommand {
    pub fn new(car_id: CarId, parking_id: i32) -> ParkCarCommand {
        ParkCarCommand {
            car_id,
            parking_id,
        }
    }
}

#[derive(CommandHandlerInBus)]
pub struct ParkCarCommandHandler {
    parking_repository: Rc<ParkingRepository>,
}

impl ParkCarCommandHandler {
    pub fn new(parking_repository: Rc<ParkingRepository>) -> ParkCarCommandHandler {
        ParkCarCommandHandler {
            parking_repository
        }
    }
}

impl CommandHandler<ParkCarCommand> for ParkCarCommandHandler {
    fn handle(&self, command: &ParkCarCommand) -> Events {
        let mut parking = self.parking_repository.find_by_id(command.parking_id).unwrap();
        Ok(vec![parking.park_a_new_car(command.car_id.clone())])
    }
}