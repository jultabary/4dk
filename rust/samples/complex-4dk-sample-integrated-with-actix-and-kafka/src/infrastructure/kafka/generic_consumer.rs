use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::Consumer;
use kafka::error::Error as KafkaError;
use log::{info, trace};
use crate::infrastructure::kafka::config::KafkaConfig;


pub fn consume_messages(mut kafka_config: KafkaConfig, topic: String, controller: fn(message: &str)) -> Result<(), KafkaError> {
    let brokers_url = kafka_config.move_broker_url();
    let mut con = Consumer::from_hosts(brokers_url)
        .with_topic(topic)
        .with_group(kafka_config.get_group())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            trace!("No messages available right now.");
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                controller(std::str::from_utf8(m.value).unwrap());
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}