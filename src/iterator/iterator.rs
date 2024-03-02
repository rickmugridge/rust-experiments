/*
    Notice how we can take a &mut Box of a dyn as argument to receiving_iterator().
    That's because we have a mut ref to the Box, we also can mutably next() that iter.
 */
fn receiving_iterator(iter: &mut Box<dyn Iterator<Item=usize>>) -> Option<usize> {
    iter.next()
}

fn passing_iterator() -> Option<usize> {
    // Notice how we need the let here, to be clear that we're interested in  the trait,
    // not the specific struct involved.
    // The compiler is fussier here then with the same issue in iterator_in_struct for some reason.
    let mut boxed: Box<dyn Iterator<Item=usize>> = Box::new((2..).into_iter());
    receiving_iterator(&mut boxed)
}


#[cfg(test)]
mod tests {
    use crate::iterator::iterator::{passing_iterator, passing_iterator_ref, passing_iterator_ref2};

    #[test]
    fn iter() {
        assert_eq!(passing_iterator(), Some(2),
        )
    }
    #[test]
    fn ref_first() {
        assert_eq!(passing_iterator_ref(), Some(2),
        )
    }

    #[test]
    fn ref_second() {
        assert_eq!(passing_iterator_ref2(), Some(2),
        )
    }
}

/*
    Notice how we could instead:
      o Box the &mut dyn trait in iter argument to receiving_iterator()
      o And also explicitly deref the iter argument to get at the &mut
      o And we can't use deref() instead of (*iter)
      o And in passing_iterator() we need to be explicit about the &mut
 */

fn receiving_iterator_ref(iter: Box<&mut dyn Iterator<Item=usize>>) -> Option<usize> {
    (*iter).next()
}

fn passing_iterator_ref() -> Option<usize> {
    let iter: &mut dyn Iterator<Item=usize> = &mut (2..).into_iter();
    receiving_iterator_ref(Box::new(iter))
}

fn passing_iterator_ref2() -> Option<usize> {
    // This eg shows that we don't need to declare iter first, as in fn above
    receiving_iterator_ref(Box::new(&mut (2..).into_iter()))
}

