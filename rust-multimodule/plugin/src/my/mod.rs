mod private;
pub mod public;

pub fn my_fn() {
    crate::Plugin;
    crate::my::private::my_private_fn();
    crate::my::public::my_public_fn();
}


