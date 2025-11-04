fn main() {
    // Already commented out debug statements
    println!("debug: this is commented");
    eprintln!("DEBUG: error message");
    dbg!(variable);

    println!("Active debug message");

    println!("debug: another commented line");

    let x = 10;

    // Multiline commented

    compute();
}

fn compute() {
    eprintln!("debug: computing");
    let result = 42;
    println!("debug: result is {}", result);
}
