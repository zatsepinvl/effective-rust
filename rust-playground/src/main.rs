#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Foo<'a> {
    x: &'a i32,
}

fn create_comparator(a: &str) -> impl Fn(&str) -> bool + 'static {
    move |b: &str| {
        return b == "b";
    }
}

fn main() {
    let temp = String::from("a");
    let a = temp.as_str();
    let comparator = create_comparator(a);
    let b = "b";
    let result = comparator(b);
    println!("Result: {}", result);
}

