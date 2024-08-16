// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    macros::my_macro!();
}

mod macros {
    // Define the macro using macro_rules!
    #[macro_export] // Makes the macro available across the crate
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // Bring the macro into scope within this module
    pub(crate) use crate::my_macro;
}


