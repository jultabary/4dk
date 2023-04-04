use std::sync::Arc;
use dddk_core::dddk::aliases::Events;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_bus::CommandBus;
use sqlx::{Pool, Postgres};
use crate::block_on;

pub struct PersistenceMiddleware {
    pool: Arc<Pool<Postgres>>,
    next_command_bus: Box<dyn CommandBus>
}

impl PersistenceMiddleware{
    pub fn new(pool: Arc<Pool<Postgres>>, next_command_bus: Box<dyn CommandBus>) -> Self {
        PersistenceMiddleware { pool, next_command_bus }
    }
}

impl CommandBus for PersistenceMiddleware {
    fn dispatch<'b>(&self, command: &'b dyn Command) -> Events {
        let transaction_result = block_on(self.pool.begin());
        match transaction_result.is_ok() {
            true => {
                let events = self.next_command_bus.dispatch(command);
                let transaction = transaction_result.unwrap();
                let _ = block_on(transaction.commit());
                events
            }
            false => {
                panic!("Fail to open a db transaction");
            }
        }
    }
}
