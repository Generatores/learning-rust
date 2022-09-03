// A function can be at any order of the code, it doesnt need to be declarated before using it
fn main() {
    my_first_function();

    // Declaratinga second function, a number/integer should have before a &, instead of 3 it should be passed as &3
    my_second_function("first", &3);
}

// the functions should have snake case
// instead of myFirstFunction it should be my_first_function
fn my_first_function() {
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
    );
}

fn my_second_function(first_argument: &str, second_argument: &i128) {
    println!(
        "the first argument is here {} and the second here {}!",
        first_argument, second_argument
    );
}
