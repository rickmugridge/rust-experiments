#[cfg(test)]
mod tests {
    #[test]
    fn capture_i32() {
        fn call_closure<C: Fn() -> i32>(c: C) -> i32 {
            c()
        }

        let i = 42;
        let capture_i/**/ = || i;
        assert_eq!(call_closure(capture_i), i);
    }

    #[test]
    fn capture_i32_ref() {
        fn call_closure<'a, C: Fn() -> &'a i32>(c: C) -> &'a i32 {
            c()
        }

        let i = &42;
        let capture_i/**/ = || i;
        assert_eq!(call_closure(capture_i), i);
    }

    #[test]
    fn ref_closure_itself() {
        fn call_closure(c: & dyn Fn() -> i32) -> i32 {
            c()
        }

        let i = 42;
        let capture_i/**/ = &|| i;
        assert_eq!(call_closure(capture_i), i);
    }

    #[test]
    fn struct_with_ref_closure() {
        struct ClosureRef<'a>{
            c: &'a dyn Fn() -> i32
        }
        fn make_closure_ref(c: & dyn Fn() -> i32) -> ClosureRef {
            ClosureRef{c}
        }

        let i = 42;
        let capture_i/**/ = &|| i;
        let closure_ref = make_closure_ref(capture_i);
        assert_eq!((closure_ref.c)(), i);
    }
}
