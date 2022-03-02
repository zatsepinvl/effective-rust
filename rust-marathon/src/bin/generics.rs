trait Trait {}

struct Bar {}

impl Trait for Bar {}

struct Foo {}

impl Trait for Foo {}

// https://doc.rust-lang.org/reference/types/impl-trait.html#differences-between-generics-and-impl-trait-in-return-position
// Params
// impl Trait in argument position
// In argument position, impl Trait is very similar in semantics to a generic type parameter.
fn param_impl_trait(foo: impl Trait) {}

fn param_generic<T: Trait>(foo: T) {}

// Return position
// impl Trait in return position =  Abstract return types
fn return_without_box_dyn() -> impl Fn(i32, i32) -> i32 {
    |a, b| a + b
}

fn return_box_dyn() -> Box<dyn Fn(i32, i32) -> i32> {
    Box::new(|a, b| a + b)
}


// With impl Trait, unlike with a generic type parameter,
// the function chooses the return type,
// and the caller cannot choose the return type.
fn return_impl_trait(arg: impl Trait) -> impl Trait {
    arg
}

fn return_generic<T: Trait>(arg: T) -> T {
    arg
}

fn main() {
    param_impl_trait(Foo {});
    param_generic::<Foo>(Foo {});

    return_without_box_dyn();
    return_box_dyn();

    // r1 is impl Trait
    let r1 = return_impl_trait(Foo {});

    // Wont compile
    // let r1_1 = return_impl_trait(Foo{}) as Foo;
    //    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    //let r1_1 = return_impl_trait(Foo{}) as Foo;

    // r2 is Foo
    let r2 = return_generic::<Foo>(Foo {});
}
