use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

fn main() {
    hello_world();
    comments();
    formatting();
    debug();
    display();
    display_test_case_list();
}

fn hello_world() {
    println!("Hello, world!");
}

fn comments() {
    // Comments
    /**
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     **/
    fn some_commented_func() {}
    some_commented_func();
}

//  Full list of formatting traits: https://doc.rust-lang.org/std/fmt/#formatting-traits
fn formatting() {
    println!("\n=====FORMATTING=====");

    println!("{} days", 31);
    println!("{who} {what} {when} {wheree}",
             who = "the lazy dog",
             what = "barked",
             when = "yesterday",
             wheree = "in the garden"
    );

    println!("{formatted:b}", formatted = 4);

    println!("{aligned:_>width$}", aligned = "aligned", width = 10);

    println!("{number:_>width$}", number = 1, width = 10);

    let pi: f64 = 3.141592;

    println!("{:.3}", pi);

    #[derive(Debug)]
    struct CustomStructure(i32);

    println!("Debug: `{:?}` ", CustomStructure(3));
    println!("Escape curly braces {{{}}}", "some_value");

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f,
                   "RGB ({r}, {g}, {b}) 0x{r:>02X}{g:>02X}{b:>02X}",
                   r = self.red, g = self.green, b = self.blue
            )
        }
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }

    println!("======\n");
}

fn debug() {
    println!("\n=====DEBUG=====");

    #[derive(Debug)]
    struct CustomStructure(i32);

    #[derive(Debug)]
    struct Deep(CustomStructure);
    println!("Deep:Debug:Pretty {:#?}", Deep(CustomStructure(1)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let person = Person { name, age };
    println!("Person:Debug: {:?}", person);
    println!("Person:Debug:Pretty {:#?}", person);

    println!("======\n");
}

fn display() {
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl Display for Complex {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

fn display_test_case_list() {
    #[derive(Debug)]
    struct MyError(String);

    impl Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for MyError {}

    fn do_for_result(err: bool) -> Result<i32, MyError> {
        if err { Err(MyError(String::from("some error"))) } else { Ok(1) }
    }

    fn deal_with_for_result() -> Result<i32, MyError> {
        let i = do_for_result(true)?;
        Ok(i)
    }

    let result = deal_with_for_result();
    println!("Result: {:?}", result);

    struct List(Vec<i32>);
    impl Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;
            for (index, v) in vec.iter().enumerate() {
                if index != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", index, v)?;
            }

            write!(f, "]")
        }
    }

    let list = List(vec![1, 2, 3]);
    println!("List:Display: {}", list);
}
