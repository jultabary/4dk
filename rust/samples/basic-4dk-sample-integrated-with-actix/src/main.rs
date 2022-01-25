use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use actix_web::{get, App, HttpServer, Responder, web};
use dddk_core::Bus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use log::LevelFilter;
use crate::logger::SimpleLogger;
use crate::usecases::commands::create_foo_command::{CreateFooCommand, CreateFooCommandHandler};

mod usecases;
mod logger;

#[get("/foo")]
async fn index(context: web::Data<RefCell<Context>>) -> impl Responder {
    let command = CreateFooCommand {};
    let _result = context.get_ref().borrow().bus.dispatch_command(&command);
    format!("Hello World !!")
}

pub struct Context {
    bus: Bus,
}

impl Context {
    pub fn new() -> Context {
        let a_command_handler = CreateFooCommandHandler {};
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(a_command_handler));

        let bus = Bus::new(command_handlers, Vec::new(), Vec::new());
        Context {
            bus
        }
    }

    pub fn get_bus(&self) -> &Bus {
        &self.bus
    }
}

static LOGGER: SimpleLogger = SimpleLogger {};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    HttpServer::new(
        || {
            let context = RefCell::new(Context::new());
            App::new()
                .service(index)
                .data(context)
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}