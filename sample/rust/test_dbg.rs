fn main() {
    let x = 5;
    let y = 10;

    // dbg! macro examples
    dbg!(x);
    dbg!(y);
    dbg!(x + y);

    // dbg! in expressions
    let result = dbg!(x * 2);

    // dbg! with references
    let name = String::from("test");
    dbg!(&name);

    // dbg! with struct fields
    let point = Point { x: 1, y: 2 };
    dbg!(point.x);
    dbg!(&point);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
