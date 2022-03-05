use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn list<T>(vals: Vec<T>) -> List<T> {
    vals.into_iter().rev().fold(
        Nil,
        move |prev, val| Cons(val, Box::new(prev)),
    )
}

fn main() {
    let l1 = Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(
                    Nil),
            )))
        ),
    );

    let l2 = list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    println!("{:?}", l1);
    println!("{:?}", l2);
}
