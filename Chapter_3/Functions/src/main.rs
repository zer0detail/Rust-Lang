fn main() {

    // Calling a function with one arg
    another_function(5);

    // Calling a function with two args, one int and one char
    print_labeled_measurement(5, 'h');

    // The next block is playing with statements vs expressions. 
    // Declare x to be 5. This is a statement and ends with ;
    let _x = 5;
    // declare Y to be the result of the code block, this does not return a value and so
    // is a statement and ends with ;
    let y = {
        // Declare x to be 3, a statement and so ends with ;
        let x = 3;
        // an expression that returns the value of x + 1 so it doesn't need ; as it's not a statement
        x + 1
    };
    println!("The value of y is: {}", y);

    // using a function that returns a value
    let x = five();
    println!("The value of x is: {}", x);

    // pass in an argument and save the return value
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// Function that takes one parameter, a signed 32 bit int and prints its value
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}


// Function that takes two params, a signed 32 bit int and a char then prints them out
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}


// Function that returns a 32bit signed int. There's no need to name returned values
fn five() -> i32 {
    // This is an expression that evaluates to 5. No ; needed. 
    5
}

// Function that takes on i32 and returns an i32 that is +1 to the input
fn plus_one(x: i32) -> i32 {
    // expression, no semicolons here.
    x + 1
}