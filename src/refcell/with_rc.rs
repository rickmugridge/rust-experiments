use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
struct Entry {
    entries: Vec<usize>,
}

impl Entry {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { entries: vec![] }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let cell = Entry::new();
        assert_eq!(cell.borrow().entries, vec![]);
    }

    #[test]
    fn borrow_mut() {
        let cell = Entry::new();
        cell.borrow_mut().entries.push(1);
        cell.borrow_mut().entries.push(2);
        assert_eq!(cell.borrow().entries, vec![1, 2]);
    }


    #[test]
    fn clone() {
        let cell = Entry::new();
        cell.borrow_mut().entries.push(5);
        let cell2 = cell.clone();  // Gives us a new Rc to the RefCell
        cell.borrow_mut().entries.push(6);
        cell2.borrow_mut().entries.push(7);
        assert_eq!(cell.borrow().entries, vec![5, 6, 7]);
        assert_eq!(cell2.borrow().entries, vec![5, 6, 7]);
    }
}

/*
 So this is a good approach to sharing a reference with dynamic mut access
 */