
pub fn run_formatted_print() {
  println!("Hello, world!");

  // In general, the `{}` will be automatically replaced with any
  // arguments. These will be stringified.
  println!("{} days", 31);

  // Positional arguments can be used. Specifying an integer inside `{}`
  // determines which additional argument will be replaced. Arguments start
  // at 0 immediately after the format string.
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments.
  println!("{subject} {verb} {object}",
           object="the lazy dog",
           subject="the quick brown fox",
           verb="jumps over");

  // Different formatting can be invoked by specifying the format character
  // after a `:`.
  println!("Base 10:               {}",   69420);
  // 69420
  println!("Base 2 (binary):       {:b}", 69420);
  // 10000111100101100
  println!("Base 8 (octal):        {:o}", 69420);
  // 207454
  println!("Base 16 (hexadecimal): {:x}", 69420);
  // 10f2c
  println!("Base 16 (hexadecimal): {:X}", 69420);
  // 10F2C

  // You can right-justify text with a specified width. This will
  // output "    1". (Four white spaces and a "1", for a total width of 5.)
  println!("{number:>5}", number=1);

  // You can pad numbers with extra zeroes,
  println!("{number:0>5}", number=1);
  // 00001
  // and left-adjust by flipping the sign. This will output "10000".
  println!("{number:0<5}", number=1);
  // 10000

  // You can use named arguments in the format specifier by appending a `$`.
  println!("{number:0>width$}", number=1, width=5);

  // Rust even checks to make sure the correct number of arguments are used.
  println!("My name is {0}, {1} {0}", "Bond", "James",);

  // Only types that implement fmt::Display can be formatted with `{}`. User-
  // defined types do not implement fmt::Display by default.

  #[allow(dead_code)] // disable `dead_code` which warn against unused module
  struct Structure(i32);

  // This will not compile because `Structure` does not implement
  // fmt::Display.
  // println!("This struct `{}` won't print...", Structure(3));

  // For Rust 1.58 and above, you can directly capture the argument from a
  // surrounding variable. Just like the above, this will output
  // "    1", 4 white spaces and a "1".
  let number: f64 = 1.0;
  let width: usize = 5;
  println!("{number:>width$}");

  let pi: f32 = 3.141592;
  let decimal_points = 3;
  println!("Pi is roughly {pi:decimal_points$}");
}

pub fn run_formatted_print_debug() {
  // This structure cannot be printed either with `fmt::Display` or
  // with `fmt::Debug`.
  #[allow(dead_code)]
  struct UnPrintable(i32);

  // The `derive` attribute automatically creates the implementation
  // required to make this `struct` printable with `fmt::Debug`.
  #[derive(Debug)]
  struct DebugPrintable(i32);

  // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
  // is a structure which contains a single `i32`.
  #[derive(Debug)]
  struct Structure(i32);

  // Put a `Structure` inside of the structure `Deep`. Make it printable
  // also.
  #[derive(Debug)]
  struct Deep(Structure);

  // Printing with `{:?}` is similar to with `{}`.
  println!("{:?} months in a year.", 12);
  println!("{1:?} {0:?} is the {actor:?} name.",
    "Slater",
    "Christian",
    actor="actor's",
  );

  // `Structure` is printable!
  println!("Now {:?} will print!", Structure(3));

  // The problem with `derive` is there is no control over how
  // the results look. What if I want this to just show a `7`?
  println!("Now {:?} will print!", Deep(Structure(7)));

  #[allow(dead_code)]
  #[derive(Debug)]
  struct Person<'a> {
    name: &'a str,
    age: u8,
  }

  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };

  // Pretty print
  println!("{:#?}", peter);
}
