
pub fn run_1_2_formatted_print() {
  println!("---------- 1.2 - Formatted print start ----------");
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
  println!("---------- 1.2 - Formatted print end  ----------");
}

pub fn run_1_2_1_formatted_print_debug() {
  println!("---------- 1.2.1 - Formatted print debug start ----------");
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
  println!("---------- 1.2.1 - Formatted print debug end ----------");
}

pub fn run_1_2_2_formatted_print_display() {
  println!("---------- 1.2.2 - Formatted print display start ----------");
  // Import (via `use`) the `fmt` module to make it available.
  use std::fmt;

  // Define a structure for which `fmt::Display` will be implemented. This is
  // a tuple struct named `Structure` that contains an `i32`.
  struct Structure(i32);

  // To use the `{}` marker, the trait `fmt::Display` must be implemented
  // manually for the type.
  impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Write strictly the first element into the supplied output
      // stream: `f`. Returns `fmt::Result` which indicates whether the
      // operation succeeded or failed. Note that `write!` uses syntax which
      // is very similar to `println!`.
      write!(f, "{}", self.0)
    }
  }

  // A structure holding two numbers. `Debug` will be derived so the results can
  // be contrasted with `Display`.
  #[derive(Debug)]
  struct MinMax(i64, i64);

  // Implement `Display` for `MinMax`.
  impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Use `self.number` to refer to each positional data point.
      write!(f, "({}, {})", self.0, self.1)
    }
  }

  // Define a structure where the fields are nameable for comparison.
  #[derive(Debug)]
  struct Point2D {
    x: f64,
    y: f64,
  }

  // Similarly, implement `Display` for `Point2D`.
  impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Customize so only `x` and `y` are denoted.
      write!(f, "x: {}, y: {}", self.x, self.y)
    }
  }

  let minmax = MinMax(0, 14);

  println!("Compare structures:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!("The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range,
  );

  let point = Point2D { x: 3.3, y: 7.2 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  // Error. Both `Debug` and `Display` were implemented, but `{:b}`
  // requires `fmt::Binary` to be implemented. This will not work.
  // println!("What does Point2D look like in binary: {:b}?", point);

  #[derive(Debug)]
  struct Complex {
    real: f64,
    imag: f64,
  }

  impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} + {}i",
        self.real,
        self.imag,
      )
    }
  }

  let complex = Complex{
    real: 3.3,
    imag: 7.2,
  };

  println!("Display: {}", complex);
  println!("Debug: {:?}", complex);
  println!("---------- 1.2.2 - Formatted print display end ----------");
}

pub fn run_1_2_2_1_formatted_print_display_list() {
  println!("---------- 1.2.2 - Formatted print display list start ----------");
  use std::fmt; // Import the `fmt` module.

  // Define a structure named `List` containing a `Vec`.
  struct List(Vec<i32>);

  impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Extract the value using tuple indexing,
      // and create a reference to `vec`.
      let vec = &self.0;

      write!(f, "[")?;

      // Iterate over `v` in `vec` while enumerating the iteration
      // count in `count`.
      for (count, v) in vec.iter().enumerate() {
        // For every element except the first, add a comma.
        // Use the ? operator to return on errors.
        if count != 0 {
          write!(f, ", ")?;
        }
        write!(f, "{}: {}", count, v)?;
      }

      // Close the opened bracket and return a fmt::Result value.
      write!(f, "]")
    }
  }

  let v = List(vec![1, 2, 3]);

  println!("{}", v);
  println!("---------- 1.2.2 - Formatted print display list end ----------");
}

pub fn run_1_2_3_formatted_print_formatting() {
  println!("---------- 1.2.2 - Formatted print display formatting start ----------");
  use std::fmt::{self, Formatter, Display};

  struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
  }

  impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
      let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

      // `write!` is like `format!`, but it will write the formatted string
      // into a buffer (the first argument).
      write!(f, "{}: {:.3}°{} {:.3}°{}",
        self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
  }

  #[allow(dead_code)]
  #[derive(Debug)]
  struct Colour {
    red: u8,
    green: u8,
    blue: u8,
  }

  impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
        self.red,
        self.green,
        self.blue,
      )
    }
  }

  for city in [
    City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    City { name: "Oslo", lat: 59.95, lon: 10.75 },
    City { name: "Vancouver", lat: 49.25, lon: -123.1 },
  ] {
    println!("{}", city);
  }
  for color in [
    Colour { red: 128, green: 255, blue: 90 },
    Colour { red: 0, green: 3, blue: 254 },
    Colour { red: 0, green: 0, blue: 0 },
  ] {
    // Switch this to use {} once you've added an implementation
    // for fmt::Display.
    println!("{}", color);
  }
  println!("---------- 1.2.2 - Formatted print display formatting end ----------");
}
