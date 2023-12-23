
use crate::core::domain::repositories::hello_world::{
  run_formatted_print,
  run_formatted_print_debug,
  run_formatted_print_display,
  run_formatted_print_display_list,
};

pub fn run_hello_world_set() {
  run_formatted_print();
  run_formatted_print_debug();
  run_formatted_print_display();
  run_formatted_print_display_list();
}