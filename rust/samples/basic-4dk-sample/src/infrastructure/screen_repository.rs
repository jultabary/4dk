pub struct ScreenRepository {}

impl ScreenRepository {
    pub fn new() -> ScreenRepository {
        ScreenRepository {}
    }

    pub fn refresh_screen(&self) {
        println!("Refresh screen of available Spots !!")
    }
}