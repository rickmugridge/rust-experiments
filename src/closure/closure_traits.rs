/*
    "Which trait is implemented is decided by what the closure does with the captured variable,
    not how it is captured." -- https://hashrust.com/blog/a-guide-to-closures-in-rust/
 */

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn fn_() {
        let clo = |a, b| a + b;
        assert_eq!(clo(2, 5), 7);
    }

    #[test]
    fn multiple_capture() {
        fn call_closure<C: Fn()>(c: C) {
            c();
            c();
        }

        let i = 42;
        // Both closures implement 'Fn':
        let capture_nothing = || println!("@17 I capture nothing");
        let capture_i/**/ = || println!("@18 I capture i immutably: {i}");
        call_closure(capture_nothing);
        call_closure(capture_i);
    }

    #[test]
    fn fn_mut() {
        fn call_closure<C: FnMut()>(mut c: C) { // c needs to be mutable due to i += 1
            c();
            c();
        }

        let mut i = 42;
        call_closure(|| {
            i += 1;
            println!("@36 {i}")
        });

        let j = 43;
        call_closure(|| println!("@40 {j}")); // Fn closure is also accepted
    }

    #[test]
    fn fn_mut_accepts_fn() {
        fn call_closure<C: FnMut()>(mut c: C) { // c needs to be mutable
            c();
            c();
        }

        let j = 43;
        call_closure(|| println!("@40 {j}")); // Fn closure is also accepted
    }

    #[test]
    fn fn_once() {
        fn call_closure_once<C: FnOnce()>(c: C) {
            c();
        }

        let s = "42".to_string();
        call_closure_once(|| { // This is a FnOnce due to the drop()
            println!("@54 Dropping {s}");
            drop(s);
        });
    }

    #[test]
    fn fn_once_accepts_fn() {
        fn call_closure_once<C: FnOnce()>(c: C) {
            c();
        }

        let i = 42;
        call_closure_once(|| println!("@74 {i}")); // a Fn
    }

    #[test]
    fn fn_once_accepts_fn_mut() {
        fn call_closure_once<C: FnOnce()>(c: C) {
            c();
        }

        let mut s = "fox".to_string();
        let fn_mut_closure = || { // a FnMut
            s.push_str("es");
        };
        call_closure_once(fn_mut_closure);
        assert_eq!(s, "foxes".to_string());
    }

    #[test]
    fn fn_once_moves_captured_variable() {
        let s = "42".to_string();
        let mut strings = Vec::new();
        let capture_s: &dyn FnOnce() = &move || {
            strings.push(s);//moves s out of the closure to the strings vec
        };
        let _c: &dyn FnOnce() = capture_s; // todo Don't know how to use that!!
        let _cc: &dyn FnOnce() = capture_s.deref();
        // assert_eq!(s, "42".to_string()); // Can't, as moved
        // assert_eq!(strings, vec!["42".to_string()]); // Can't, as moved
    }
}