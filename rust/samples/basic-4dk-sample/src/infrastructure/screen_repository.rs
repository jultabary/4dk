use log::info;

pub struct ScreenRepository {}

impl ScreenRepository {
    pub fn new() -> ScreenRepository {
        ScreenRepository {}
    }

    pub fn refresh_screen(&self) {
        info!("FAKE EXTERNAL SYSTEM: Refresh screen of available Spots !!")
    }
}