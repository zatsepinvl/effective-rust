use std::mem;

#[allow(dead_code)]
fn main() {
    structures();
    enums();
    enums_use_keyword();
    enums_c_like();
    enums_linked_list();
    constants();
}

fn structures() {
    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // A unit struct
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    let pair = Pair(1, 1.0);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer, decimal);
    println!("Pair occupies {} bytes", mem::size_of_val(&pair));
}

fn enums() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeePress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("PageLoad"),
            WebEvent::PageUnload => println!("PageUpload"),
            WebEvent::KeePress(c) => println!("KeePress {}", c),
            WebEvent::Paste(s) => println!("Paste {}", s),
            WebEvent::Click { x, y } => println!("Click {} {}", x, y),
        }
    }

    inspect(WebEvent::PageLoad);
    inspect(WebEvent::PageUnload);
    inspect(WebEvent::KeePress('a'));
    inspect(WebEvent::Paste(String::from("str")));
    inspect(WebEvent::Click { x: 1, y: 1 });

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // Creates a type alias
    type Operation = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Operation {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y
            }
        }
    }

    println!("Operation::Add : {}", Operation::Add.run(1, 1));
}

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

fn enums_use_keyword() {
    println!("\n# Enums use keyword");
    #[derive(Debug)]
    enum Work {
        Civilian,
        Soldier,
    }

    println!("Work: {:?}", Work::Civilian);
    println!("Status: {:?}", Status::Poor);

    use Work::*;
    use crate::Status::{Poor};
    println!("Work: {:?}", Civilian);
    println!("Status: {:?}", Poor);
}

fn enums_c_like() {
    println!("\n# Enums c-like");
    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

fn enums_linked_list() {
    println!("\n# enums_linked_list");

    use List::*;

    enum List {
        // Node: Tuple struct that wraps an element and a pointer to the next node
        Node(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }

    impl List {
        fn new() -> List {
            Nil
        }

        // Consume a list, and return the same list with a new element at its front
        fn prepend(self, elem: u32) -> List {
            Node(elem, Box::new(self))
        }

        // Return the length of the list
        fn len(&self) -> u32 {
            match self {
                // Can't take ownership of the tail, because `self` is borrowed;
                // instead take a reference to the tail
                Node(_, tail) => 1 + tail.len(),
                // Base Case: An empty list has zero length
                Nil => 0
            }
        }

        // Return representation of the list as a (heap allocated) string
        fn stringify(&self) -> String {
            match self {
                Node(val, tail) => format!("{} -> {}", val, tail.stringify()),
                Nil => format!("nil")
            }
        }
    }

    let list = List::new().prepend(1).prepend(2).prepend(3);
    println!("{}", list.stringify());
}

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn constants() {
    println!("\n# constants");
    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    //THRESHOLD = 5;

    static mut MUTE_STATIC: i32 = 1;
    unsafe {
        println!("MUTE_STATIC: {}", MUTE_STATIC);
        MUTE_STATIC = 2;
        println!("MUTE_STATIC: {}", MUTE_STATIC);
    }
}