use log::info;

pub struct GateRepository {}

impl GateRepository {
    pub fn new() -> GateRepository {
        GateRepository {}
    }

    pub fn open_gate(&self) {
        info!("FAKE EXTERNAL SYSTEM: Open Entry Gate !!")
    }
}