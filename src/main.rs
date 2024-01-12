
// TODO: Rust by Example 3.2: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

// TODO: After completing Rust by Example, the next area I want to explore is graphics programming: https://www.rastertek.com/tutindex.html 

mod core;

use crate::core::domain::use_cases::run_1_hello_world_set;
use crate::core::domain::use_cases::run_2_primitives_set;
use crate::core::domain::use_cases::run_3_custom_types_set;

fn main() {
  run_1_hello_world_set::main();
  run_2_primitives_set::main();
  run_3_custom_types_set::main();
}
