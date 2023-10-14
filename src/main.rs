use std::fmt::{Display, Formatter};
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}
struct ListIterator<'a, T> {
    cur: &'a List<T>
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a List<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let List::Cons(_, next) = self.cur {
            let ret = self.cur;
            self.cur = next.as_ref();
            Some(ret)
        } else {
            None
        }
    }
}

impl<T: Display + Clone> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "List[").expect("Can't happen");
        self.iter()
            .for_each(|x| { write!(f, "{}, ", x.get().unwrap())
                .expect("Can't happen"); } );
        write!(f, "]\n")
    }
}

impl<T: Clone> List<T> {
    fn new(val: T) -> List<T> {
        List::Cons(val, Box::new(List::Nil))
    }

    fn get(self: &Self) -> Option<T> {
        match self {
            List::Nil => None,
            List::Cons(val, _) => Some(val.clone())
        }
    }

    fn append(mut self: Self, val: T) -> List<T> {
        let mut cur = &mut self;

        while let List::Cons(_, next) = cur {
            match **next {
                List::Nil => { **next = List::Cons(val, Box::new(List::Nil)); break; },
                _ => ()
            }
            cur = next;
        }
        self
    }

    fn iter(&self) -> ListIterator<T> {
        ListIterator { cur: self }
    }
}

fn main() {
    let lst = List::new(1)
        .append(2)
        .append(3)
        .append(4)
        .append(5);
    println!("{lst}");
}
