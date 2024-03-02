#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn borrow() {
        let rc= Rc::new(32);
        assert_eq!(*rc, 32);
    }

    #[test]
    fn get_mut() {
        let mut rc= Rc::new(32);
        let rc2 = Rc::get_mut(&mut rc);
        if let Some(r) = rc2 {
            *r = 64;
        }
        assert_eq!(*rc, 64);
    }

    // #[test]
    // fn cannot_borrow_rc_more_than_once_at_a_time() {
    //     let mut rc= Rc::new(32);
    //     let rc2 = Rc::get_mut(&mut rc);
    //     if let Some(r) = rc2 {
    //         let rc3 = Rc::get_mut(&mut rc);
    //         *r = 64;
    //     }
    //     assert_eq!(*rc, 64);
    // }
}