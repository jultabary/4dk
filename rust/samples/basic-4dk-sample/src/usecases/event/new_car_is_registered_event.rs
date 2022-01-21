use std::any::Any;
use dddk_core::dddk::event::event::Event;
use crate::domain::car::CarId;

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

impl Event for ANewCarIsParkedEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}