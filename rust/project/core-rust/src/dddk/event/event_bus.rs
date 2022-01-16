use crate::dddk::event::event::Event;

pub trait EventBus {
    fn dispatch<'b>(&self, event: &'b dyn Event);
}