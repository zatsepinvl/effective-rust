#![feature(never_type)]
fn main() {
    functions();
    methods();
    closures();
    high_order_function();
    divergin_functions();
}

fn functions() {
    println!("\n# functions");

    // can be invoked before declaration
    fuzzbuzz_to(1);

    fn fuzzbuzz_to(n: u32) {
        for n in 1..n + 1 {
            fuzzbuzz(n);
        }
    }

    // Functions that "don't" return a value, actually return the unit type `()`
    fn fuzzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Function that returns a boolean value
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            // Corner case, early return
            return false;
        }

        // This is an expression, the `return` keyword is not necessary here
        lhs % rhs == 0
    }
}

fn methods() {
    println!("\n# methods");

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    // Implementation block, all `Point` associated functions & methods go in here
    impl Point {
        // This is an "associated function" because this function is associated with
        // a particular type, that is, Point.
        //
        // Associated functions don't need to be called with an instance.
        // These functions are generally used like constructors.
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Another associated function, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }

        // This is a method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Point`
        fn add(&self, point: &Point) -> Point {
            Point {
                x: self.x + point.x,
                y: self.y + point.y,
            }
        }

        // This method "consumes" the resources of the caller object
        // `self` desugars to `self: Self`
        fn destroy(self) {
            // Destructure `self`
            let Point { x, y } = self;
            println!("Destroying Point({}, {})", x, y);
            // `first` and `second` go out of scope and get freed
        }
    }

    let point1 = Point::origin();
    let point2 = Point::new(1.0, 1.0);
    let point3 = point1.add(&point2);
    println!("point3: {:?}", point3);
    point1.destroy();
}

fn closures() {
    println!("\n# closures");

    fn inc(i: i32) -> i32 { i + 1 }
    let inc_closure_annotated = |i: i32| -> i32 { i + 1 };
    let inc_closure_inferred = |i| i + 1;

    let i = 1;
    println!("inc: {}", inc(i));
    println!("inc_closure_annotated: {}", inc_closure_annotated(i));
    println!("inc_closure_inferred: {}", inc_closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    {
        // # Capturing
        use std::mem;

        let color = String::from("green");

        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last time.
        //
        // `println!` only requires arguments by immutable reference so it doesn't
        // impose anything more restrictive.
        let print = || println!("`color`: {}", color);

        let _reborrow = &color;
        print();

        let mut count = 0;
        // A closure to increment `count` could take either `&mut count` or `count`
        // but `&mut count` is less restrictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
        // calling the closure mutates the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // Call the closure using a mutable borrow.
        inc();

        // The closure still mutably borrows `count` because it is called later.
        // An attempt to reborrow will lead to an error.
        // let _reborrow = &count;
        // ^ TODO: try uncommenting this line.
        inc();

        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // possible to reborrow without an error
        let _count_reborrowed = &mut count;


        // A non-copy type.
        let movable = Box::new(3);

        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        // `consume` consumes the variable so this can only be called once.
        consume();
        // consume();
        // ^ TODO: Try uncommenting this line.

        // `Vec` has non-copy semantics.
        let haystack = vec![1, 2, 3];

        // Using move before vertical pipes forces closure to take ownership of captured variables:
        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // println!("There're {} elements in vec", haystack.len());
        // ^ Uncommenting above line will result in compile-time error
        // because borrow checker doesn't allow re-using variable after it
        // has been moved.
    }
}

fn high_order_function() {
    println!("\n# high_order_function");

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let a = &1;
    let &b = a;

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n_sq| n_sq <= upper)
            .filter(|&n_sq| is_odd(n_sq))
            .fold(0, |acc, n_sq| acc + n_sq);
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn divergin_functions() {
    println!("\n# divergin_functions");

    fn foo() -> ! {
        panic!("This call never returns.");
    }

    fn some_fn() {
        ()
    }

    let a: () = some_fn();
    println!("This function returns and you can see this line.");

    //let x: ! = panic!("This call never returns.");
    //println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}