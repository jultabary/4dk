use std::any::{Any, TypeId};
use crate::dddk::event::event::Event;

pub trait EventForTest: Event {
    fn get_id(&self) -> i32;

    fn get_type_id(&self) -> TypeId;
}

pub struct AnEvent {
    pub id: i32,
}

impl AnEvent {
    pub fn new(id: i32) -> AnEvent {
        AnEvent { id }
    }
}

impl Event for AnEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EventForTest for AnEvent {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<AnEvent>()
    }
}

unsafe impl Send for AnEvent {}

pub struct AnotherEvent {
    pub id: i32,
}

impl AnotherEvent {
    pub fn new(id: i32) -> AnotherEvent {
        AnotherEvent { id }
    }
}

impl Event for AnotherEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EventForTest for AnotherEvent {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<AnotherEvent>()
    }
}

unsafe impl Send for AnotherEvent {}
