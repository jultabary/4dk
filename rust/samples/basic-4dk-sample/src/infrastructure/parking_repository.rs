use crate::domain::parking::Parking;

pub struct ParkingRepository {
    parkings: Vec<Parking>,
}

impl ParkingRepository {
    pub fn new(parkings: Vec<Parking>) -> ParkingRepository {
        ParkingRepository {
            parkings
        }
    }

    pub fn find_by_id(&self, id: i32) -> Option<Parking> {
        if let Some(parking) = self.parkings.iter().find(|parking| { parking.get_id() == id }) {
            return Some(parking.clone());
        }
        None
    }
}