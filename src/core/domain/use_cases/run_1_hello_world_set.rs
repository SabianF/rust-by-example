
// TODO: Current step 1.2.3 https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html 

use crate::core::domain::repositories::hello_world::{
  run_1_2_formatted_print,
  run_1_2_1_formatted_print_debug,
  run_1_2_2_formatted_print_display,
  run_1_2_2_1_formatted_print_display_list,
  run_1_2_3_formatted_print_formatting,
};

pub fn main() {
  println!("========== 1 - Hello World start ==========");
  run_1_2_formatted_print();
  run_1_2_1_formatted_print_debug();
  run_1_2_2_formatted_print_display();
  run_1_2_2_1_formatted_print_display_list();
  run_1_2_3_formatted_print_formatting();
  println!("========== 1 - Hello World end ==========");
}