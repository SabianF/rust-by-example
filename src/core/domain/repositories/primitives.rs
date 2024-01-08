use core::fmt;

pub fn run_2_1_literals_and_operators() {
  println!("---------- 2.1 - Literals and operators start ----------");
  // Integer addition
  println!("1 + 2 = {}", 1u32 + 2);

  // Integer subtraction
  println!("1 - 2 = {}", 1i32 - 2);

  // Scientific notation
  println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

  // Short-circuiting boolean logic
  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

  // Use underscores to improve readability!
  println!("One million is written as {}", 1_000_000u32);
  println!("---------- 2.1 - Literals and operators end ----------");
}

pub fn run_2_2_tuples() {
  println!("---------- 2.2 Tuples start ----------");

  fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

  // The following struct is for the activity.
  #[derive(Debug)]
  struct Matrix(f32, f32, f32, f32);

  // A tuple with a bunch of different types.
  let long_tuple = (
    1u8,
    2u16,
    3u32,
    4u64,
    -1i8,
    -2i16,
    -3i32,
    -4i64,
    0.1f32, 
    0.2f64,
    'a',
    true
  );

  // Values can be extracted from the tuple using tuple indexing.
  println!("Long tuple first value: {}", long_tuple.0);
  println!("Long tuple second value: {}", long_tuple.1);

  // Tuples can be tuple members.
  let tuple_of_tuples = (
    (
      1u8,
      2u16,
      2u32,
    ),
    (
      4u64,
      -1i8,
    ),
    -2i16,
  );

  // Tuples are printable.
  println!("tuple of tuples: {:?}", tuple_of_tuples);

  // But long Tuples (more than 12 elements) cannot be printed.
  //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  //println!("Too long tuple: {:?}", too_long_tuple);

  let pair = (1, true);
  println!("Pair is {:?}", pair);

  println!("The reversed pair is {:?}", reverse(pair));

  // To create one element tuples, the comma is required to tell them apart
  // from a literal surrounded by parentheses.
  println!("One element tuple: {:?}", (5u32,));
  println!("Just an integer: {:?}", (5u32));

  // Tuples can be destructured to create bindings.
  let tuple = (1, "hello", 4.5, true);

  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);

  impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      writeln!(f, "({}, {})", self.0, self.1)?;
      writeln!(f, "({}, {})", self.2, self.3)
    }
  }

  println!("Activity: Add the `fmt::Display` trait to the `Matrix` struct:");
  println!("{}", matrix);

  println!("---------- 2.2 Tuples end ----------");
}
