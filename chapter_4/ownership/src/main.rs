fn main() {
    // String is created (heap) and s gains ownership
    let s = String::from("Hello");

    // s loses ownership and it's given to the variable inside "takes_ownership"
    // s is no longer valid after this line unless you return something back into s
    takes_ownership(s);

    // This line makes the program uncompilable as s no longer owns the value
    // println!("{}", s);


    //// These calls below show that integers aren't constrained by the same ownership
    //// rules as things like strings. This is because integers are of a known fixed size
    //// so it's trivial to just make another copy on the stack for the new variable.
    //// Strings (heap ones, not literals) like the one above, are mutable and you cannot
    //// just make an easy copy on the stack because they arent even stored on the stack
    //// they are on the heap and can grow and shrink in size, so copies are not made.
    //// the implication with the previous stuff is that you can pass an int (like x below)
    //// into a function and it will make a copy for the function, meaning you retain ownership
    //// of the variable in the original scope too.
    // x owns the value 5
    let x = 5;

    // this pass a copy of x to makes_copy
    makes_copy(x);

    // x is still valid down here. HOWEVER because the x that was passed into makes_copy()
    // was a copy, the update to the variable (+=1) only applied that the new x. 
    // this x is still 5.
    println!("x: {}", x);
    
}

fn takes_ownership(some_string: String) {
    // some_string took ownership from s of the "hello" value
    println!("{}", some_string);


}

fn makes_copy(mut some_integer: i32) {
    some_integer += 1;
    // some_integer has a copy of x, so x is still valid
    println!("{}", some_integer)
}