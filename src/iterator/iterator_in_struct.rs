/*
     Notice with HoldsIterator:
       o It just boxes the dyn Iterator directly
       o We need a &mut of that struct to call next() on it
     Notice how we don';'t need the &mut ref inside the box in HoldsIteratorWithLifetime
 */

struct HoldsIterator {
    iter: Box<dyn Iterator<Item=usize>>,
}

impl HoldsIterator {
    fn new(iter: Box<dyn Iterator<Item=usize>>) -> Self {
        Self { iter }
    }
    fn next(&mut self) -> Option<usize> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::iterator::iterator_in_struct::{HoldsIteratorWithLifetime, HoldsIterator};

    #[test]
    fn holds_iterator() {
        let in_iter = (2..).into_iter();
        let holder = &mut HoldsIterator::new(Box::new(in_iter));
        assert_eq!(holder.next(), Some(2));
        assert_eq!(holder.next(), Some(3));
    }

    #[test]
    fn with_lifetime() {
        let in_iter = &mut (2..).into_iter();
        let holder = &mut HoldsIteratorWithLifetime::new(Box::new(in_iter));
        assert_eq!(holder.next(), Some(2));
        assert_eq!(holder.next(), Some(3));
    }
}

struct HoldsIteratorWithLifetime<'a> {
    iter: Box<&'a mut dyn Iterator<Item=usize>>,
}

impl<'a> HoldsIteratorWithLifetime<'a> {
    fn new(iter: Box<&'a mut dyn Iterator<Item=usize>>) -> Self {
        Self { iter }
    }
    fn next(&mut self) -> Option<usize> {
        self.iter.next()
    }
}
