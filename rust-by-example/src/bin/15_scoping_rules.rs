fn main() {
    raii();
    ownership_and_moves();
}


fn raii() {
    // Rust enforces RAII (Resource Acquisition Is Initialization)
    // https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization
    //  so whenever an object goes out of scope, its destructor is called and its owned resources are freed.

    {
        // Allocate an integer on the heap
        let _box = Box::new(3);

        // `_box1` is destroyed here, and memory gets freed
    }

    // The notion of a destructor in Rust is provided through the Drop trait.
    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let _drop = ToDrop;
}

fn ownership_and_moves() {
    // resource can have ONLY ONE owner
    // Stack variables
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // Heap variables
    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);
    // *Move* `a` into `b`
    let _b = a;
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);

    fn partial_moves() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person { name: String::from("John"), age: Box::new(12) };
        // `name` is moved out of person, but `age` is referenced
        let Person { name, ref age } = person;

        println!("The person's age is {}", age);
        println!("The person's name is {}", name);

        // Error! borrow of partially moved value: `person` partial move occurs
        //println!("The person struct is {:?}", person);

        // `person` cannot be used but `person.age` can be used as it is not moved
        println!("The person's age from person struct is {}", person.age);
    }
    partial_moves();
}

fn borrowing() {
    // ToDo
}

fn lifetimes() {
    // ToDo
}
