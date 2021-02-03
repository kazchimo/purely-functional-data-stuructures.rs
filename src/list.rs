use crate::list::List::{Nil, Cons};

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    fn nil() -> List<T> {
        Nil
    }

    fn one(a: T) -> List<T> {
        Cons(a, Box::new(Nil))
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
        assert!(List::<i32>::nil().is_empty());
        assert!(!List::one(0).is_empty());
    }
}

