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
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rectangle;
    let Point { x: top_edge, y: left_edge } = top_left;
    let Point { x: bottom_edge, y: right_edge } = bottom_right;

    let area = (top_edge - bottom_edge).abs() * (left_edge - right_edge).abs();

    area
}

fn square(point: Point, float: f32) -> Rectangle {
    let Point { x: left, y: lower } = point;
    let top_left = Point { x: left, y: lower + float };
    let bottom_right = Point { x: left + float, y: lower };

    Rectangle { top_left: top_left, bottom_right: bottom_right }
}

fn main() {
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
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: top_edge, y: left_edge },
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

    println!("_rectangle: {}, {}", _rectangle.top_left.x, _rectangle.top_left.y);

    let area = rect_area(_rectangle);
    println!("Area of rectangle: {}", area);

    let pt = Point { x: 1.0, y: 1.0 };
    let sqr = square(pt, 2.0);
    println!("Square: ({}, {}), ({}, {})",
             sqr.top_left.x, sqr.top_left.y, sqr.bottom_right.x, sqr.bottom_right.y);
}
