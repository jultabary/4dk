use std::any::Any;
use dddk_core::dddk::event::event::Event;
use dddk_macro::Event;
use crate::domain::car::CarId;

#[derive(Event)]
pub struct ANewCarIsParkedEvent {
    car_id: CarId,
}

impl ANewCarIsParkedEvent {
    pub fn new(car_id: CarId) -> ANewCarIsParkedEvent {
        ANewCarIsParkedEvent {
            car_id
        }
    }

    pub fn get_new_car_id(&self) -> &CarId {
        &self.car_id
    }
}
