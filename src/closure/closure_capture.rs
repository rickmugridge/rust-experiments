#[cfg(test)]
mod tests {
    #[test]
    fn simple_closure_without_capture() {
        let clo = |a, b| a + b;
        assert_eq!(clo(2, 5), 7);
    }

    #[test]
    fn simple_capture() {
        let b = 42;
        let clo = |a| a + b;
        println!("@13 {b}");
        assert_eq!(clo(2), 44);
    }

    #[test]
    fn simple_capture_with_string() {
        let b = "42".to_string();
        let clo = || b;
        // println!("{b}"); // Not permitted because the String has been moved (or, with i32 as Copy)
        assert_eq!(clo(), "42");
    }

    #[test]
    fn capture_with_ref_string() {
        let b = &"42".to_string();
        let clo = || b;
        println!("{b}"); // Permitted because the &String can be moved (as a ref)
        assert_eq!(*clo(), "42");
    }

    #[test]
    fn returns_closure() {
        fn make_adder(a: i32) -> impl Fn(i32) -> i32 { // todo When is "impl" needed?
            move |b| a + b
        }
        let adder = make_adder(5);
        assert_eq!(adder(2), 7);
    }

    #[test]
    fn mutably_borrow() {
        let mut animal = "fox".to_string();
        let mut capture = || { animal.push_str("es"); };
        // println!("{animal}"); // Not permitted as it is already mutably borrowed
        capture();
        assert_eq!(animal, "foxes".to_string());
    }

    #[test]
    fn move_as_borrowed() {
        let animal = "fox".to_string();
        let ref_animal = &animal;
        let capture = move || { println!("{ref_animal}"); };
        println!("@48 {animal}"); // Permitted as only the reference is borrowed
        println!("@49 {ref_animal}"); // Permitted as only the reference is borrowed
        capture();
        println!("@51 {animal}"); // Permitted as only the reference is borrowed
    }

    #[test]
    fn fn_mut() {
        fn create_pluralizer(mut animal: String) -> impl FnMut() {
            move || { //Need to move, as closure may outlive function
                animal.push_str("es");
                println!("@59 Pluralized animal: {animal}");
            }
        }

        let mut pluralize_fox = create_pluralizer("fox".to_string());
        pluralize_fox();
    }
}
