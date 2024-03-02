#[derive(Debug, PartialEq, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        !self.is_left()
    }
}

impl<L> Either<L, L> {
    pub fn map<U, F>(self, f: F) -> Either<U, U> where F: FnOnce(L) -> U {
        match self {
            Either::Left(e) => Either::Left(f(e)),
            Either::Right(e) => Either::Right(f(e)),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::functor::either::Either;

    #[test]
    fn is_left() {
        let left: Either<i32, bool> = Either::Left(1);
        let right: Either<i32, bool> = Either::Right(true);
        assert_eq!(left.is_left(), true);
        assert_eq!(right.is_left(), false);
    }

    #[test]
    fn is_right() {
        let left: Either<i32, bool> = Either::Left(1);
        let right: Either<i32, bool> = Either::Right(true);
        assert_eq!(left.is_right(), false);
        assert_eq!(right.is_right(), true);
    }

    #[test]
    fn maps() {
        let left: Either<i32, i32> = Either::Left(1);
        let right: Either<i32, i32> = Either::Right(3);
        assert_eq!(left.map(|a| a + 1), Either::Left(2));
        assert_eq!(right.map(|a| a + 1), Either::Right(4));
    }
}