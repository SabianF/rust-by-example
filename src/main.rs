// TODO: Current step 1.2.1 https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html

mod core;

use crate::core::domain::repositories::hello_world;

fn main() {
    hello_world::run_formatted_print();
}
