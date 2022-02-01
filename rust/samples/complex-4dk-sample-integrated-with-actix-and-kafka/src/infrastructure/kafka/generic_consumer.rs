use std::sync::Arc;
use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::Consumer;
use kafka::error::Error as KafkaError;
use log::trace;
use crate::bus_config::Context;
use crate::infrastructure::kafka::config::KafkaConfig;


pub fn consume_messages(mut kafka_config: KafkaConfig,
                        topic: String,
                        context: Arc<Context>,
                        controller: fn(message: &str, context: &Arc<Context>)) -> Result<(), KafkaError> {
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
                controller(std::str::from_utf8(m.value).unwrap(), &context);
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}