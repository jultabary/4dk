pub struct CarId {
    id: i32,
}

impl CarId {
    pub fn new(id: i32) -> CarId {
        CarId { id }
    }
}

impl Clone for CarId {
    fn clone(&self) -> Self {
        CarId::new(self.id)
    }
}

impl Copy for CarId {}