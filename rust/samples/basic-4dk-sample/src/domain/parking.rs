use std::sync::Arc;
use dddk_core::dddk::event::event::Event;
use crate::domain::car::CarId;
use crate::usecases::event::new_car_is_registered_event::ANewCarIsParkedEvent;
use crate::usecases::event::refused_to_register_car_event::RefusedToRegisterCarEvent;

pub struct Parking {
    id: i32,
    max_spots: i32,
    parked_cars: Vec<CarId>,
}

impl Parking {
    pub fn new(id: i32, max_places: i32, parked_cars: Vec<CarId>) -> Parking {
        Parking { id, max_spots: max_places, parked_cars }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_max_spot_available(&self) -> i32 {
        self.max_spots
    }

    pub fn get_available_spots(&self) -> i32 {
        self.max_spots - self.parked_cars.len() as i32
    }

    pub fn park_a_new_car(&mut self, car_id: CarId) -> Arc<dyn Event> {
        if self.get_available_spots() > 0 {
            self.parked_cars.push(car_id.clone());
            Arc::new(ANewCarIsParkedEvent::new(car_id))
        } else {
            Arc::new(RefusedToRegisterCarEvent::new(car_id))
        }
    }
}

impl Clone for Parking {
    fn clone(&self) -> Self {
        Parking::new(self.max_spots, self.id, self.parked_cars.clone())
    }
}
