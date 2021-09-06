fn main() {
    variable_bindings();
    mutability();
    scope_and_shadowing();
    declare_first();
    freezing();
}

fn variable_bindings() {
    println!("\n# variable_bindings");
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
}

fn mutability() {
    println!("\n# mutability");

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    //_immutable_binding += 1;
}

fn scope_and_shadowing() {
    println!("\n# scope_and_shadowing");

    // This binding lives in the main function
    let long_lived_binding = 1;
    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding: u8 = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // Error! `short_lived_binding` doesn't exist in this scope
    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);
        // This binding *shadows* the outer one
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);
    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn declare_first() {
    println!("\n# declare_first");

    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    // Error! Use of uninitialized binding
    //println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding: {}", another_binding);
}

fn freezing() {
    println!("\n# freezing");

    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;
        // Error! `_mutable_integer` is frozen in this scope
        //_mutable_integer = 50;
        // FIXME ^ Comment out this line
        // `_mutable_integer` goes out of scope
    }
    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}