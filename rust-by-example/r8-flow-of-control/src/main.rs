#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {
    if_else();
    looop();
    whiile();
    for_and_range();
    maatch();
    if_let();
}

fn if_else() {
    println!("\n# if_else");

    let n = 6;
    if n < 5 {
        println!("n < 5");
    } else if n == 5 {
        println!("n == 5");
    } else {
        println!("n > 5");
    }

    let n_bigger_than_five = if n <= 5 { false } else { true };

    println!("Is n bigger than five? - {}", n_bigger_than_five);
}

fn looop() {
    println!("\n# loop");

    let mut count = 0u32;

    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }

    // Nesting and labels
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            //break;
            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");


    // Returning from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // Stop and return value
            break counter * 2;
        }
    };
    println!("Counter: {}, Result: {}", counter, result);
    assert_eq!(result, 20);
}

fn whiile() {
    println!("\n# whiile");

    // A counter variable
    let mut n = 1;
    // Loop while `n` is less than 101
    while n < 31 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        // Increment counter
        n += 1;
    }
}

fn for_and_range() {
    println!("\n# for_and_range");

    // for and range
    // `n` will take the values: 1, 2, ..., 30 in each iteration
    for n in 1..31 {
        //...FizzBuzz here
        if n == 31 { println!("31!") }
    }

    // `n` will take the values: 1, 2, ..., 31 in each iteration
    for n in 1..=31 {
        //...FizzBuzz here
        if n == 31 { println!("31!") }
    }

    // for and iterators
    // iter()
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_inter()
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // causes borrow of moved value: `names` error
    //println!("names: {:?}", names);

    // iter_mute()
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}

fn maatch() {
    println!("\n# maatch");

    let number = 13;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };
    println!("{} -> {}", boolean, binary);

    // Destructing
    // tuples
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`
    println!("Tell me about {:?}", triple);
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    // enums
    enum Color { Red, Blue, Green, RGB(u32, u32, u32) }
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        // Don't need another arm because all variants have been examined
    }

    // pointers/ref
}

fn if_let() {
    println!("\n# if_let");
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }


    #[derive(Debug)]
    enum MyEnum { Color(i16, i16, i16), AnotherStruct(i16, i16, i16)}

    let another_struct = MyEnum::AnotherStruct(0, 0, 0);

    if let MyEnum::Color(a,b,c) = another_struct   {
        println!("{:?} is a color ({},{},{})", another_struct, a, b, c);
    } else {
        println!("{:?} is NOT a color", another_struct);
    }
}

fn while_let() {
    println!("\n# while_let");
}
