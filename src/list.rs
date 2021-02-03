use crate::list::List::{Nil, Cons};

#[derive(PartialEq)]
enum List<'a, T> {
    Nil,
    Cons(&'a T, Box<&'a List<'a, T>>),
}

impl<T> List<'_, T> {
    fn empty() -> List<'static, T> {
        Nil
    }

    fn one(a: &T) -> List<T> {
        Cons(a, Box::new(&Nil))
    }

    fn cons<'a>(x: &'a T, s: &'a List<T>) -> List<'a, T> {
        Cons(x, Box::new(s))
    }

    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            Cons(_, _) => false
        }
    }

    fn head(&self) -> &T {
        match self {
            Nil => panic!("Empty List"),
            Cons(x, _) => x
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
        assert!(!List::one(&0).is_empty());
    }

    #[test]
    fn cons() {
        assert!(List::cons(&1, &Nil) == Cons(&1, Box::new(&Nil)));
        assert!(List::cons(&1, &List::cons(&2, &Nil)) == Cons(&1, Box::new(&Cons(&2, Box::new(&Nil)))))
    }

    #[test]
    #[should_panic]
    fn fail_head() {
        List::<i32>::empty().head();
    }

    fn head() {
        assert_eq!(List::one(&1).head(), &1)
    }
}

