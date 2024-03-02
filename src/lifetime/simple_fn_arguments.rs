/*
    Here we start with some simple examples of passing lifetime arguments to a function
 */
fn no_lifetimes(p: &u32) -> &u32 {
    p
}

fn unnecessary_lifetimes<'a>(p: &'a u32) -> &'a u32 {
    p
}

fn necessary_lifetimes<'a, 'b>(p: &'a u32, _q: &'b u32) -> &'a u32 {
    p
}


#[cfg(test)]
mod tests {
    use crate::lifetime::simple_fn_arguments::{necessary_lifetimes, no_lifetimes, unnecessary_lifetimes};

    #[test]
    fn fn_takes_no_lifetimes() {
        let i: u32 = 45;
        assert_eq!(*no_lifetimes(&i), 45);
    }

    #[test]
    fn fn_unnecessary_lifetimes() {
        let i: u32 = 45;
        assert_eq!(*unnecessary_lifetimes(&i), 45);
    }

    #[test]
    fn fn_necessary_lifetimes() {
        let i: u32 = 45;
        let j: u32 = 67;
        assert_eq!(*necessary_lifetimes(&i, &j), 45);
    }
}