fn main() {
    // println!("Starting application");
    // println!("debug: initialization started");

    let x = 42;
    // println!("Value: {}", x);

    // eprintln!("error: something went wrong");
    // eprintln!("DEBUG: detailed error info");

    // print!("inline message");
    // eprint!("inline error");

    // Normal computation
    let result = calculate(10, 20);
    // println!("Result: {}", result);
}

fn calculate(a: i32, b: i32) -> i32 {
    // println!("debug: calculating {} + {}", a, b);
    a + b
}
