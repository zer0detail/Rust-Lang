fn main() {
    // S1 is a String created on the heap as it is not a literal (str) and can be expected
    // to grow in size.
    let mut s1 = String::from("hello");

    // Rather than pass in the variable s1 itself and lose ownership of it, we can pass
    // in a reference to s1 so that calculate_length can still make use of it, but we still
    // own it and can use it after the call.
    let len = calculate_length(&s1);

    // s1 is still valid since we only passed in a reference and retained ownership
    println!("The length of '{}' is {}", s1, len);

    change(&s1);
    change_fixed(&mut s1);

    println!("New string: {}", s1);

    // This fails, as we try to borrow two mutable references to the same variable at the same time. 
    // this is not allowed as it could lead to race condition errors where the string is changed by both vars
    // which are expecting the same older string but the second reference to make the change is actually
    // dealing with an entirely new string that the first reference just changed, which leads to 
    // unexpected results. 
    
    // let s2 = &mut s1;
    // let s3 = &mut s1;


    // This is fine though because  the {} put s2 into a seperate scope
    {
        let _s2 = &mut s1;
    } // once we get here s2 is no longer in scope and so we can create s3 as another mutable
    // reference to s1 because s2 no longer exists. 
    let _s3 = &mut s1;

    let mut s = String::from("hello");

    // This is fine, we can declare an immutable reference to s
    let r1 = &s;
    // This is also fine as all references to s are ready only (immutable) so nothing
    // is going to change unexpectedly for these references while theyre reading
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // This is not ok, we declare a mutable reference to s, meaning r1 and r2 cant reasonably
    // be sure that they will be reading s correctly as r3 might have changed it 
    let r3 = &mut s;

    // The problem actually only pops up here though once we use those references. 
    
    // println!("{} {} {}", r1 ,r2, r3);

    // we can fix that by just not using the immutable references anymore once we declare the 
    // mutable reference. reference are no longer in scope after their last use. so this is fine
    println!("no longer using r1 or r2: {}", r3);

    
    //// Now we try to create a dangling pointer and show that rust wont let us
    let reference_to_nothing = dangle();
    
}

fn calculate_length(s: &String) -> usize {
    // s is a variable that contains the address of s1 which allows it to access
    // s1s underlying string on the heap and do operations on it like calculating the length.
    s.len()
}

fn change(_some_string: &String) {
    
    //// this line attempts to modify a borrowing variable and will result in a compiler error
    //// this is because references are immutable by default, just like variables.
    //_some_string.push_str(", world");

} 

fn change_fixed(some_string: &mut String) {
    some_string.push_str(", world");
}


// This function return a reference to a variable that instantly goes out of scope
// this would lead to a dangle reference in the calling function since the returned
// pointer no longer points to anything (since s goes out of scope and gets cleaned up)
// so rust complains and refuses to compile our dangly code. 
fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}