use crate::list::List::{Nil, Cons};

#[derive(PartialEq)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    fn empty() -> List<T> {
        Nil
    }

    fn one(a: T) -> List<T> {
        Cons(a, Box::new(Nil))
    }

    fn cons(x: T, s: List<T>) -> List<T> {
        Cons(x, Box::new(s))
    }

    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            Cons(_, _) => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List::{Nil, Cons};
    use crate::list::List;

    #[test]
    fn is_empty() {
        assert!(List::<i32>::empty().is_empty());
        assert!(!List::one(0).is_empty());
    }

    #[test]
    fn cons() {
        assert!(List::cons(1, Nil) == Cons(1, Box::new(Nil)));
        assert!(List::cons(1, List::cons(2, Nil)) == Cons(1, Box::new(Cons(2, Box::new(Nil)))))
    }
}

