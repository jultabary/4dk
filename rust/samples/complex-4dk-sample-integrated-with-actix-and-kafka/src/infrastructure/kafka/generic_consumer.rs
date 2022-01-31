use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::Consumer;
use kafka::error::Error as KafkaError;
use crate::infrastructure::kafka::config::KafkaConfig;


pub fn consume_messages(mut kafka_config: KafkaConfig, topic: String) -> Result<(), KafkaError> {
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
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, m.value);
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}