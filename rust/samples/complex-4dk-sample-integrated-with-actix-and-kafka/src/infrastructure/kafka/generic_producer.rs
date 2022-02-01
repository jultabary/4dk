use std::sync::{Arc, Mutex};
use std::time::Duration;
use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};
use kafka::error::Error as KafkaError;
use crate::KafkaConfig;


pub fn create_kafka_producer(
    mut kafka_config: KafkaConfig,
) -> Result<Arc<Mutex<Producer>>, KafkaError> {
    let brokers = kafka_config.move_broker_url();
    let producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;
    Ok(Arc::new(Mutex::new(producer)))
}


pub fn send_message(producer: Arc<Mutex<Producer>>, topic: &'static str, message: String) {
    let _result = producer.lock().unwrap().send(&Record::from_value(topic, message.into_bytes()));
}