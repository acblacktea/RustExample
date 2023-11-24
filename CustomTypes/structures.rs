#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f64{
    let point1 = &rect.top_left;
    let point2 = &rect.bottom_right;
    let point1_x = point1.x;
    let point1_y = point1.y;
    let point2_x = point2.x;
    let point2_y = point2.y;

    return (point2_y as f64 - point1_y as f64) * (point2_x as f64 - point1_x as f64);
}

fn square(point: &Point, width: f32) -> Rectangle {
    Rectangle {
        top_left:  Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + width,
            y: point.y + width
        }
    }
}
fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: 32.0},
        bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);


    println!("{:?}", square(&point, 5.0));
}