fn main() {
    // To print text on the command line we use the println! function
    println!("Hello, world!");

    // If we add a {} inside a "" the values to be displayed will be passed as arguments
    println!("{} and {}", 1, 2);

    let value: &str = "first value";

    // a value cant be displayed alone in a println!
    // this displays an error
    //println!(value);

    // this is the proper way
    println!(
        "To display a variable it should be inside a curly bracket '{}'",
        value
    )
}
