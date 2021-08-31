#![feature(generic_associated_types)]


struct Container<T> {
    my_data: T
}



trait RepeaterFirst<'a> {
    type Item: 'a;
    fn repeat_first(&'a self) -> Self::Item;
}



trait RepeaterSecond {
    type Item<'a> /* ??? */;
    fn repeat_second<'b>(&'b self) -> Self::Item<'b>;
}



impl<'a, T> RepeaterFirst<'a> for Container<T>
/* FIXME: first
where ???,
*/
{
    type Item = &'a T;

    fn repeat_first(&'a self) -> Self::Item {
        &self.my_data
    }
}

/* FIXME: uncomment second
impl<T> RepeaterSecond for Container<T> {
    type Item<'a> /* ??? */ = &'a T;
    fn repeat_second<'b>(&'b self) -> Self::Item<'b> {
        &self.my_data
    }
}*/


fn main() {
    let data = Container { my_data: 123 };
    for _ in 0..5 {
        println!("{:?}", data.repeat_first());
    }

    /* FIXME: uncomment second
    for _ in 0..5 {
        println!("{:?}", data.repeat_second());
    }
    */
}

// HINT: (second) consider the example declaration bellow.
// the type constructor allows as specify static lifetime.
// This is not what we want.
//
// struct Test<T: RepeaterSecond> {
//    field: <T as RepeaterSecond>::Item<'static>,
// }