
use crate::core::domain::repositories::hello_world::{
  run_1_2_formatted_print,
  run_1_2_1_formatted_print_debug,
  run_1_2_2_formatted_print_display,
  run_1_2_2_1_formatted_print_display_list,
};

pub fn run_1_hello_world_set() {
  run_1_2_formatted_print();
  run_1_2_1_formatted_print_debug();
  run_1_2_2_formatted_print_display();
  run_1_2_2_1_formatted_print_display_list();
}