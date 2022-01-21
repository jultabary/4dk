use std::any::Any;
use dddk_core::dddk::event::event::Event;
use crate::domain::car::CarId;

pub struct RefusedToRegisterCarEvent {
    car_id: CarId,
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

impl Event for RefusedToRegisterCarEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}