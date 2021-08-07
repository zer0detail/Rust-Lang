// All comments are made after writing the code and before reading the read of the section. 
// They are me taking a stab at what the line of code does or commenting on something different/noticable.
// This lets me read the code and guess at functionality before I read about it
// to help me learn. So they might be wrong, but that's the point.

// Use is the same as import in python. 
// Instead of using std.io we use :: instead. 
// Even import lines end in a semicolon.. so imports are statements I guess
use std::io;

// fn instead of function or def. No need for colons here.
fn main() {
    // away with the easy python single print call, we have println, which makes me think it'll
    // be alot like c with println, print, printf etc.
    println!("Guess the number!");

    // The ! means it's a macro, but no idea what that means yet.  I don't even want to take a guess. 
    // Just no clue.
    println!("Please input your guess.");

    // let reminds me of javascript. I guess we need it before every var declaration
    // VScode let me know that guess doesn't need to be mutable, so variables must be immutable by default unless we specify mut
    // guess is the var name, that's obvious enough
    // Rust is statically typed so String must be the type definition. 
    // Looks like type definitions are structs that have methods like new, which I assume returns an empty string. 
    // You access class/struct methods with :: and not . i'm not overly familiar with c++ but this looks like c++ style class calling
    let mut guess = String::new();

    // call the io libraries stdin method, which reads from stdin
    // We read stdin and save it to the address of guess
    io::stdin()
        // this is nuts. We newline and then tab and start with a dot? what
        // it's not function parameters, because we just called stdin() with empty parethesis.
        // yeah im not sure.
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Print out the guess we received from stdin. fingers crossed something like f strings exist. 
    println!("You guessed: {}", guess);
}