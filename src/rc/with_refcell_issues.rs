#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn borrow() {
        let rc = Rc::new(RefCell::new(32));
        assert_eq!(*rc.borrow(), 32);
    }

    #[test]
    fn borrow_mut() {
        let rc = Rc::new(RefCell::new(32));
        *rc.borrow_mut() = 64;
        assert_eq!(*rc.borrow(), 64);
    }

    #[test]
    fn borrow_mut_some() {
        let rc = Rc::new(RefCell::new(Some(32)));
        *rc.borrow_mut() = Some(64);
        assert_eq!(*rc.borrow(), Some(64));
    }

    #[test]
    fn borrow_conflict() {
        // This makes sense as the mut borrow is within the "scope" of the borrow
        let rc = Rc::new(RefCell::new(Some(32)));
        if let Some(_v) = *rc.borrow() {
            *rc.borrow_mut() = Some(64);
        }
        assert_eq!(*rc.borrow(), Some(64));
    }

    #[test]
    fn borrow_conflict_in_else() {
        // Is this a compiler bug, as the mut borrow seems to be outside the "scope" of the borrow
        let rc = Rc::new(RefCell::new(Some(32)));
        if let Some(_v) = *rc.borrow() {
        } else {
            *rc.borrow_mut() = Some(64);
        }
        assert_eq!(*rc.borrow(), Some(64));
    }

    #[test]
    fn mut_borrow_conflict() {
        // This makes sense as the borrow is within the "scope" of the mut borrow
        let rc = Rc::new(RefCell::new(Some(32)));
        if let Some(v) = rc.borrow_mut().as_mut() {
            *v = 64;
            let _v = rc.borrow();
        }
        assert_eq!(*rc.borrow(), Some(64));
    }

    #[test]
    fn mut_borrow_conflict_in_else() {
        // Interesting that this is not a compiler bug, as it's much the same as the borrow_conflict() test above
        // But see the test below, which doesn't as_mut(), which makes the difference.
        let rc = Rc::new(RefCell::new(Some(32)));
        if let Some(v) = rc.borrow_mut().as_mut() {
            *v = 64;
        } else {
            let v = rc.borrow();
            assert_eq!(*v, Some(64));
        }
        assert_eq!(*rc.borrow(), Some(64));
    }

    #[test]
    fn mut_borrow_conflict_in_else2() {
        // Is this a compiler bug, as the borrow seems to be outside the "scope" of the mut borrow
        let rc = Rc::new(RefCell::new(Some(32)));
        if let Some(v) = *rc.borrow_mut() {
            assert_eq!(v, 32);
        } else {
            let v = rc.borrow();
            assert_eq!(*v, Some(64));
        }
        assert_eq!(*rc.borrow(), Some(64));
    }
}