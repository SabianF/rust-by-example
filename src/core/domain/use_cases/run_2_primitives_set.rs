
use crate::core::domain::repositories::primitives::{
  run_2_1_literals_and_operators,
  run_2_2_tuples,
  run_2_3_arrays_and_slices,
};

pub fn main() {
  println!("========== 2 - Primitives start ==========");
  run_2_1_literals_and_operators();
  run_2_2_tuples();
  run_2_3_arrays_and_slices();
  println!("========== 2 - Primitives end ==========");
}
