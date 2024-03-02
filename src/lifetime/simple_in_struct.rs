struct Simple<'a> {
    p: &'a u32,
}

fn make_simple(p: &u32) -> Simple {
    Simple { p }
}

fn make_simple_recursively(i: u32, p: &u32) -> Simple {
    if i <= 0 {Simple { p }} else { make_simple_recursively(i-1, p)}
}

fn use_simple_locally() -> u32 {
    let i: u32 = 45;
    *make_simple(&i).p
}

#[cfg(test)]
struct Double<'a, 'b> {
    p: &'a u32,
    q: &'b u32,
}

impl<'a, 'b> Double<'a, 'b> {
    fn new(p: &'a u32, q: &'b u32) -> Self {
        Self { p, q }
    }
}

fn make_double<'a, 'b>(p: &'a u32, q: &'b u32) -> Double<'a, 'b> {
    Double { p, q }
}

#[cfg(test)]
mod tests {
    use crate::lifetime::simple_in_struct::{Double, make_double, make_simple, make_simple_recursively, Simple, use_simple_locally};

    #[test]
    fn struct_takes_a_lifetime() {
        let i: u32 = 45;
        let s = Simple { p: &i };
        assert_eq!(*s.p, 45);
    }

    #[test]
    fn struct_takes_a_lifetime_made_indirectly() {
        let i: u32 = 45;
        let s = make_simple(&i);
        assert_eq!(*s.p, 45);
    }

    #[test]
    fn struct_takes_a_lifetime_made_indirectly_recursively() {
        let i: u32 = 45;
        let s = make_simple_recursively(2, &i);
        assert_eq!(*s.p, 45);
    }

    #[test]
    fn struct_takes_a_lifetime_and_used_locally() {
        assert_eq!(use_simple_locally(), 45);
    }

    #[test]
    fn struct_takes_2_lifetimes() {
        let i: u32 = 45;
        let j: u32 = 67;
        let s = Double { p: &i, q: &j };
        assert_eq!(*s.p, 45);
        assert_eq!(*s.q, 67);
    }

    #[test]
    fn struct_takes_2_lifetimes_made_indirectly() {
        let i: u32 = 45;
        let j: u32 = 67;
        let s = make_double(&i, &j);
        assert_eq!(*s.p, 45);
        assert_eq!(*s.q, 67);
        let s = Double::new(&i, &j);
        assert_eq!(*s.p, 45);
        assert_eq!(*s.q, 67);
    }
}

/*
    Notice that we can easily create instances of these structs here, as we've got the u32's created in each test.
    But, in general, we can't easily creates a Single anywhere, because of the lifetime constraint of 'a.
    So the only way to create a Single within a function that also creates the p, q, is to fully use the Single in that fn.
 */