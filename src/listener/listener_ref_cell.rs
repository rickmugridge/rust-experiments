use std::cell::RefCell;

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
    listeners: Vec<RefCell<&'a mut dyn Listener>>,
}

impl<'a> EventCreator<'a> {
    fn new() -> Self {
        Self { listeners: vec![] }
    }

    fn fire(&mut self, value: i32) {
        self.listeners.iter_mut().for_each(|listener| listener.get_mut().happened(value))
    }

    fn attach(&mut self, listener: RefCell<&'a mut dyn Listener>) {
        self.listeners.push(listener);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    #[test]
    fn single_fire_from_bare_listener() {
        let listener1 = &mut Listener1::new();
        let mut event_creator = EventCreator::new();
        event_creator.attach(RefCell::new(listener1));
        event_creator.fire(22);
        assert_eq!(listener1.current(), 22);
    }

    #[test]
    fn double_fire_from_bare_listener() {
        let listener1 = &mut Listener1::new();
        let mut event_creator = EventCreator::new();
        event_creator.attach(RefCell::new(listener1));
        event_creator.fire(22);
        assert_eq!(listener1.current(), 22);
        // event_creator.fire(44); // When uncommented: Compiler error on line above: cannot borrow `*listener1` as immutable because it is also borrowed as mutable
    }

    #[test]
    fn double_fire_from_ref_cell_listener() {
        let bare_listener = &mut Listener1::new();
        let listener1 = RefCell::new(bare_listener);
        let mut event_creator = EventCreator::new();
        event_creator.attach(listener1);
        event_creator.fire(22);
        // assert_eq!(listener1.borrow().current(), 22); // But has been moved in attach
        // assert_eq!(bare_listener.current(), 22); // But has been moved
        // event_creator.fire(44); // Compiler error: cannot borrow `*listener1` as immutable because it is also borrowed as mutable
    }
}
// BACK