#[cfg(test)]
pub mod some_logger_for_test {
    use std::cell::RefCell;
    use log::{Level, Log, Metadata, Record};

    pub struct MockLogger {
        pub(crate) messages: RefCell<Vec<String>>,
    }

    impl Log for MockLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut message = String::new();
                message.push_str(record.level().as_str());
                message.push_str("_");
                message.push_str(&*record.args().to_string());
                self.messages.borrow_mut().push(message);
            }
        }

        fn flush(&self) {
            self.messages.borrow_mut().clear();
        }
    }

    unsafe impl Sync for MockLogger {}

    unsafe impl Send for MockLogger {}
}