#[derive(Debug, PartialEq, Clone)]
struct Entry {
    entries: Vec<usize>,
}

impl Entry {
    fn new() -> Self {
        Self { entries: vec![] }
    }
}

fn f(e: &mut Entry) {
    g(e);
    g(e);
}

fn g(e: &mut Entry) { e.entries.push(1); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let mut entry = Entry::new();
        f(&mut entry);
        f(&mut entry);
        g(&mut entry);
        assert_eq!(entry.entries, vec![1, 1, 1, 1, 1]);
    }
}