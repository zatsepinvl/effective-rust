fn main() {
    functions();
    methods();
    closures();
    high_order_function();
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

}

fn high_order_function() {
    println!("\n# high_order_function");
}