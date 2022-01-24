use std::any::Any;
use std::fmt::{Debug, Formatter};
use dddk_core::dddk::event::event::Event;
use dddk_macro::Event;
use crate::domain::car::CarId;

#[derive(Event)]
pub struct RefusedToRegisterCarEvent {
    car_id: CarId,
}

impl Debug for RefusedToRegisterCarEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefusedToRegisterCarEvent")
    }
}

impl RefusedToRegisterCarEvent {
    pub fn new(car_id: CarId) -> RefusedToRegisterCarEvent {
        RefusedToRegisterCarEvent {
            car_id
        }
    }

    pub fn get_new_car_id(&self) -> &CarId {
        &self.car_id
    }
}
