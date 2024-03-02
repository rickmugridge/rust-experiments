#[derive(Debug, PartialEq, Clone)]
pub enum Optional<T> {
    None,
    Some(T),
}

impl<T> Optional<T> {
    pub fn is_none(&self) -> bool {
        match self {
            Optional::None => true,
            Optional::Some(_) => false,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn map<U, F>(&self, f: F) -> Optional<U> where F: FnOnce(&T) -> U {
        match self {
            Optional::Some(t) => Optional::Some(f(t)),
            _ => Optional::None,
        }
    }

    pub fn filter<F>(self, f: F) -> Optional<T> where F: FnOnce(&T) -> bool {
        match self {
            Optional::Some(t) if f(&t) => Optional::Some(t),
            _ => Optional::None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::functor::optional::Optional;

    #[test]
    fn map_over_options() {
        let a = vec![Optional::<i32>::None, Optional::Some(1)];
        let result:Vec<Optional<i32>> = a.iter().map(|opt| opt.map(|a|a+1)).collect();
        assert_eq!(result, vec![Optional::<i32>::None, Optional::Some(2)]);
    }

    #[test]
    fn is_none() {
        assert_eq!(Optional::<i32>::None.is_none(), true);
        assert_eq!(Optional::Some(1).is_none(), false);
    }

    #[test]
    fn is_some() {
        assert_eq!(Optional::<i32>::None.is_some(), false);
        let some = Optional::Some(1);
        assert_eq!(some.is_some(), true);
    }

    #[test]
    fn map() {
        assert_eq!(Optional::<i32>::None.map(|a| a + 1), Optional::<i32>::None);
        assert_eq!(Optional::Some(1).map(|a| a + 1), Optional::Some(2));
    }

    #[test]
    fn filter() {
        assert_eq!(Optional::<i32>::None.filter(|a| *a > 0), Optional::<i32>::None);
        assert_eq!(Optional::Some(0).filter(|a| *a > 0), Optional::<i32>::None);
        assert_eq!(Optional::Some(1).filter(|a| *a > 0), Optional::Some(1));
    }
}