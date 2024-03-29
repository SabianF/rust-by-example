use std::fmt::Display;


pub fn run_3_1_structures() {
  // An attribute to hide warnings for unused code.
  #![allow(dead_code)]

  println!("---------- 3.1 Structures end ----------");

  #[derive(Debug)]
    struct Person {
      name: String,
      age: u8,
    }

  // A unit struct
  struct Unit;

  // A tuple struct
  struct Pair(i32, f32);

  // A struct with two fields
  struct Point {
    x: f32,
    y: f32,
  }

  // Structs can be reused as fields of another struct
  struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
  }

  // Create struct with field init shorthand
  let name = String::from("Peter");
  let age = 27;
  let peter = Person { name, age };

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a `Point`
  let point: Point = Point { x: 10.3, y: 0.4 };

  // Access the fields of the point
  println!("point coordinates: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our
  // other one
  let bottom_right = Point { x: 5.2, ..point };

  // `bottom_right.y` will be the same as `point.y` because we used that field
  // from `point`
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the point using a `let` binding
  let Point { x: left_edge, y: top_edge } = point;

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  // Instantiate a unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);

  // Add a function rect_area which calculates the area of a Rectangle
  // (try using nested destructuring).
  fn rect_area(rectangle: Rectangle) -> f32 {
    let length = rectangle.top_left.x - rectangle.bottom_right.x;
    let width = rectangle.top_left.y - rectangle.bottom_right.y;
    return length * width;
  }

  let _rect = Rectangle{
    top_left: Point{
      x: 0.0,
      y: 0.0,
    },
    bottom_right: Point{
      x: 6.9,
      y: 42.0,
    },
  };

  let _area = rect_area(_rect);

  println!("rect_area: {}", _area);

  // Add a function square which takes a Point and a f32 as arguments,
  // and returns a Rectangle with its top left corner on the point, and
  // a width and height corresponding to the f32.
  fn square(top_left: Point, size: f32) -> Rectangle {

    let bottom_right = Point {
        x: top_left.x + size,
        y: top_left.y + size,
    };

    return Rectangle {
      top_left,
      bottom_right,
    };
  }

  let _top_left = Point {
    x: 0.0,
    y: 0.0,
  };
  let _size = 6.9;
  let _square = square(_top_left, _size);

  impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Point 1: ({}, {})\nPoint 2: ({}, {})",
        self.top_left.x,
        self.top_left.y,
        self.bottom_right.x,
        self.bottom_right.y,
      )
    }
  }

  println!("Square:\n{}", _square);

  println!("---------- 3.1 Structures end ----------");
}
