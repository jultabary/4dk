use std::env;
use dotenv::dotenv;

pub struct KafkaConfig {
    broker_url: Vec<String>,
    group: String,
}

impl KafkaConfig {
    pub fn from_var_env() -> KafkaConfig {
        dotenv().ok();
        let broker_url_raw = env::var("KAFKA_URL")
            .expect("KAFKA_URL must be set");
        let group = env::var("KAFKA_GROUP_ID")
            .expect("KAFKA_GROUP_ID must be set");
        let broker_url = broker_url_raw.split(',')
            .map(|broker_raw_string| { broker_raw_string.to_string() })
            .collect::<Vec<String>>();
        KafkaConfig {
            broker_url,
            group,
        }
    }

    pub fn get_group(&self) -> String {
        self.group.clone()
    }

    pub fn get_broker_url(&self) -> Vec<String> {
        self.broker_url.clone()
    }
}

