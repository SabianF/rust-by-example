// TODO: Current step 1.2.2 https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

mod core;

use crate::core::domain::repositories::hello_world::{
  run_formatted_print,
  run_formatted_print_debug,
  run_formatted_print_display,
  run_formatted_print_display_list,
};

fn main() {
  run_formatted_print();
  run_formatted_print_debug();
  run_formatted_print_display();
  run_formatted_print_display_list();
}
