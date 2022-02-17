use std::fmt::{Debug, Formatter};
use dddk_macro::Event;
use crate::domain::car::CarId;

#[derive(Event)]
pub struct ANewCarIsParkedEvent {
    car_id: CarId,
}

impl Debug for ANewCarIsParkedEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANewCarIsParkedEvent")
    }
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
