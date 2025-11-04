fn main() {
    // Single line
    println!("debug: single line");

    // Multiline println
    println!(
        "debug: multiline message with value: {}",
        compute_value()
    );

    // Complex expression
    println!(
        "Results: x={}, y={}, sum={}",
        10,
        20,
        10 + 20
    );

    // Nested function calls
    eprintln!("debug: nested call result: {}", calculate(5, multiply(2, 3)));

    // Very long multiline
    println!(
        "DEBUG: This is a very long debug message \
         that spans multiple lines and contains \
         important information: {}",
        format_data()
    );
}

fn compute_value() -> i32 {
    dbg!(42)
}

fn calculate(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn format_data() -> String {
    String::from("formatted")
}
