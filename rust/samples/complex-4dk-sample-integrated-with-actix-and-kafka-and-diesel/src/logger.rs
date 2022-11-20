use log::{Level, Log, Metadata, Record};

pub struct SimpleLogger {}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut message = String::new();
            message.push_str(record.level().as_str());
            message.push_str(" - ");
            message.push_str(&*record.args().to_string());
            println!("{}", message);
        }
    }

    fn flush(&self) { }
}