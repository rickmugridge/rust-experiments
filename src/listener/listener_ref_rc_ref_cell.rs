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
    fn new() -> Self {
        Self { value: 0 }
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

struct EventCreator<'a> {
    listeners: Vec<Rc<RefCell<&'a mut dyn Listener>>>,
}

impl<'a> EventCreator<'a> {
    fn new() -> Self {
        Self { listeners: vec![] }
    }

    fn fire(&mut self, value: i32) {
        self.listeners.iter_mut().for_each(|listener|
            listener.borrow_mut().deref_mut().happened(value))
    }

    fn attach(&mut self, listener: Rc<RefCell<&'a mut dyn Listener>>) {
        self.listeners.push(listener);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    #[test]
    fn single_fire_from_bare_listener() {
        let bare_listener = &mut Listener1::new();
        let listener1: Rc<RefCell<&mut dyn Listener>> = Rc::new(RefCell::new(bare_listener));
        let mut event_creator = EventCreator::new();
        event_creator.attach(listener1.clone());
        event_creator.fire(22);
        assert_eq!(listener1.borrow().current(), 22);
    }

    #[test]
    fn double_fire_from_bare_listener() {
        let bare_listener = &mut Listener1::new();
        let listener1: Rc<RefCell<&mut dyn Listener>> = Rc::new(RefCell::new(bare_listener));
        let mut event_creator = EventCreator::new();
        event_creator.attach(listener1.clone());
        event_creator.fire(22);
        assert_eq!(listener1.borrow().current(), 22);
        event_creator.fire(44);
    }

    #[test]
    fn double_fire_from_ref_cell_listener() {
        let bare_listener = &mut Listener1::new();
        let listener1: Rc<RefCell<&mut dyn Listener>> = Rc::new(RefCell::new(bare_listener));
        let mut event_creator = EventCreator::new();
        event_creator.attach(listener1.clone());
        event_creator.fire(22);
        assert_eq!(listener1.borrow().current(), 22);
        assert_eq!(bare_listener.current(), 22);
        // event_creator.fire(44); // Compiler error: cannot borrow `*listener1` as immutable because it is also borrowed as mutable
    }
}
