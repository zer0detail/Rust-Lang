fn main() {
    // String is created (heap) and s gains ownership
    let s = String::from("Hello");

    // s loses ownership and it's given to the variable inside "takes_ownership"
    // s is no longer valid after this line unless you return something back into s
    takes_ownership(s);

    // This line makes the program uncompilable as s no longer owns the value
    // println!("{}", s);

    // x owns the value 5
    let x = 5;

    // this pass a copy of x to makes_copy
    makes_copy(x);

    // x is still valid down here
    
}

fn takes_ownership(some_string: String) {
    // some_string took ownership from s of the "hello" value
    println!("{}", some_string);


}

fn makes_copy(some_integer: i32) {
    // some_integer has a copy of x, so x is still valid
    println!("{}", some_integer)
}