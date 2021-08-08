// All comments are made after writing the code and before reading the read of the section. 
// They are me taking a stab at what the line of code does or commenting on something different/noticable.
// This lets me read the code and guess at functionality before I read about it
// to help me learn. So they might be wrong, but that's the point.

fn main() {
    // Initial code. Commenting it out because we "fix it" later in the lesson
    // x is defined as 5 and is not explicitly declared as mutable (mut). 
    // Trying to change x to 6 then results in inability to compile because we are changing 
    // an immutable variable. 
        // let x = 5;
        // println!("The value of x is: {}", x);
        // x = 6;
        // println!("The value of x is: {}", x);

    // declare x to be 5
    let mut x = 5;
    println!("The value of x is: {}", x);
    // reassign x to be 6. this is successful because x is mutable. 
    x = 6;
    println!("The value of x is: {}", x);   

    // declare y to be 5 (and immutable)
    let y = 5;

    // use shadowing to redeclare y to be equal to y + 1. this is fine even though y is immutable because we used let
    let y = y + 1;

    // inside this code block y becomes  12
    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {}", y);
    }
    // y is back to 6 becauses its inner scope values are lost
    println!("The value of x is: {}", y);

    // 64 bit float (the default)
    let _x = 2.0;

    // 32 bit float
    let _y: f32 = 3.0;

    // Basic addition 
    let _sum = 5 + 10;

    // Basic subtraction
    let _difference = 95.5 - 4.3;

    // Basic multiplication
    let _product = 4 * 30;

    // Basic division
    let _quotient = 56.7 / 32.3;
    let _floor = 2/3; // 0

    // Modulus 
    let _remainder = 43 % 5;

    // settings t/f bools with inferred type
    let _t = true;
    // setting t/f bools with explicit type
    let _f: bool = false;

    // EMOJIS
    let _cat = '😻';

    // tuple assignment, tuples can have various types inside but they must all be defined
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // we can unpack a tuple into vars like this
    let (_x, y, _z) = tup;
    println!("The value of y is {}", y);

    // Testing a range of other types in a tuple
    let mytup: (String, i128, u8, char) = (String::from("Hello"), 400, 12, 'A');
    println!("mytup: {} {} {} {}", mytup.1, mytup.0, mytup.3, mytup.2);

    // Declaring a binary number
    let binnum: u32 = 0b1101_1010;
    println!("{}", binnum);

    // Declaring an array of some months. Array are fixed in length, so no more months can be added. 
    let _months = ["Jan", "Feb", "Mar", "Apr" ];


    // Little code block to let you access an out of bounds index and cause the program to panic by entering any number 5 and above
    let a = [1, 2, 3, 4, 5];

    println!("please enter an array index.");

    // declare a mutable variable called index set to an empty string type
    let mut index = String::new();

    // Read in stdin from the user
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // trim and parse index to an int of a size determined by the CPU arch. Catching non int's with .expect
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
        
    // allow the out of bounds error to occur here. 
    let element = a[index];

    // This won't get reached if we input 5 or more.
    println!("The value of element at index {} is: {}",
        index, element
    );
}