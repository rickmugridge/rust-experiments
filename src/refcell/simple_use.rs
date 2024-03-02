use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
struct Entry {
    entries: Vec<usize>,
}

impl Entry {
    fn new() -> RefCell<Self> {
        RefCell::new(Self { entries: vec![] })
    }

    fn bare() -> Self {
        Self { entries: vec![] }
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
    fn replace() {
        let cell = Entry::new();
        cell.borrow_mut().entries.push(1);
        assert_eq!(cell.borrow().entries, vec![1]);
        cell.replace(Entry::bare()); // Will panic if currently borrowed
        assert_eq!(cell.borrow().entries, vec![]);
    }

    #[test]
    fn swap() {
        let cell = Entry::new();
        cell.borrow_mut().entries.push(1);
        let cell2 = Entry::new();
        cell2.borrow_mut().entries.push(2);

        cell.swap(&cell2); // Will panic if either is currently borrowed, or the sameRefCell
        assert_eq!(cell.borrow().entries, vec![2]);
        assert_eq!(cell2.borrow().entries, vec![1]);
    }

    #[test]
    fn try_borrow() {
        let cell = Entry::new();
        cell.borrow_mut().entries.push(5);
        let entry = cell.try_borrow().unwrap();
        assert_eq!(entry.entries, vec![5]);
        match cell.try_borrow() {
            Ok(entry) => assert_eq!(entry.entries, vec![5]),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn clone() {
        let cell = Entry::new();
        cell.borrow_mut().entries.push(5);
        let cell2 = cell.clone();  // This clones the RefCell and its contents
        cell.borrow_mut().entries.push(6);
        assert_eq!(cell.borrow().entries, vec![5, 6]);
        assert_eq!(cell2.borrow().entries, vec![5]);
    }

    #[test]
    fn ref_cell_moves_its_argument() {
        let bare_cell = Entry::bare();
        let cell = RefCell::new(bare_cell);
        cell.borrow_mut().entries.push(5);
        assert_eq!(cell.borrow().entries, vec![5]);
        // assert_eq!(bare_cell.entries, vec![5]); // Compiler error, as it's been moved
    }
}