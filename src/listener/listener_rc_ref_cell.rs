use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

trait Listener {
    fn happened(&mut self, value: i32);
    fn current(&self) -> i32;
}

struct Listener1 {
    value: i32,
}

impl Listener1 {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value: 0 }))
    }
}

impl Listener for Listener1 {
    fn happened(&mut self, value: i32) {
        self.value = value;
    }

    fn current(&self) -> i32 {
        self.value
    }
}

struct EventCreator {
    listeners: Vec<Rc<RefCell<dyn Listener>>>,
}

impl EventCreator {
    fn new() -> Self {
        Self { listeners: vec![] }
    }

    fn fire(&mut self, value: i32) {
        self.listeners.iter_mut().for_each(|listener|
            listener.borrow_mut().deref_mut().happened(value))
    }

    fn attach(&mut self, listener: Rc<RefCell<dyn Listener>>) {
        self.listeners.push(listener);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fire() {
        let mut event_creator = EventCreator::new();
        let listener1 = Listener1::new();
        event_creator.attach(listener1.clone());
        event_creator.fire(22);
        assert_eq!(listener1.borrow().current(), 22);
    }

    #[test]
    fn fire_twice() {
        let mut event_creator = EventCreator::new();
        let listener1 = Listener1::new();
        event_creator.attach(listener1.clone());
        event_creator.fire(22);
        assert_eq!(listener1.borrow().current(), 22);
        event_creator.fire(44);
        assert_eq!(listener1.borrow().current(), 44);
    }

    #[test]
    fn two_listeners() {
        let mut event_creator = EventCreator::new();
        let listener1 = Listener1::new();
        event_creator.attach(listener1.clone());
        let listener2 = Listener1::new();
        event_creator.attach(listener2.clone());
        event_creator.fire(22);
        assert_eq!(listener1.borrow().current(), 22);
        assert_eq!(listener2.borrow().current(), 22);
        event_creator.fire(44);
        assert_eq!(listener1.borrow().current(), 44);
        assert_eq!(listener2.borrow().current(), 44);
    }
}
