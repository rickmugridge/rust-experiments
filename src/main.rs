mod refcell;
mod mutability;
mod traits;
mod closure;
mod listener;
mod rc;
mod iterator;
mod lifetime;
mod functor;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // So we can run all the tests at once
    #[test]
    fn t() {
        assert_eq!(true, true);
    }
}
