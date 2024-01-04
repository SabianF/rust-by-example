
mod core;

use crate::core::domain::use_cases::run_1_hello_world_set;
use crate::core::domain::use_cases::run_2_primitives_set;

fn main() {
  run_1_hello_world_set::main();
  run_2_primitives_set::main();
}
