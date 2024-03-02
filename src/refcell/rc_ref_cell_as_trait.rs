use std::cell::RefCell;
use std::rc::Rc;

trait Composed {
    fn new()-> Self;
    fn get(&self) -> u32;
    fn is_zero(&self) -> bool;
    fn increment(&mut self);
}

impl Composed for Rc<RefCell<u32>> {
    fn new() -> Self {
        Rc::new(RefCell::new(0))
    }

    fn get(&self) -> u32 {
        *self.borrow()
    }

    fn is_zero(&self) -> bool {
        *self.borrow() == 0
    }

    fn increment(&mut self) {
        *self.borrow_mut() += 1;
    }
}

impl Composed for Rc<RefCell<f64>> {
    fn new() -> Self {
        Rc::new(RefCell::new(0.0))
    }

    fn get(&self) -> u32 {
        *self.borrow() as u32
    }

    fn is_zero(&self) -> bool {
        *self.borrow() == 0.0
    }

    fn increment(&mut self) {
        *self.borrow_mut() += 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let cell: Rc<RefCell<u32>> = Composed::new();
        assert_eq!(cell.is_zero(), true);
    }

    #[test]
    fn borrow_64() {
        let cell: Rc<RefCell<f64>> = Composed::new();
        assert_eq!(cell.is_zero(), true);
    }

    #[test]
    fn increment() {
        let mut cell =  Rc::new(RefCell::new(0));
        cell.increment();
        assert_eq!(cell.get(), 1);
    }

    #[test]
    fn increment_f64() {
        let mut cell =  Rc::new(RefCell::new(0.0));
        cell.increment();
        assert_eq!(cell.get(), 1);
    }
}