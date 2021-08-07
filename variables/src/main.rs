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

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);   

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y);
}