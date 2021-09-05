// This is all about using slices to retrieve a slice of a string that will remain valid
// as opposed to returning an index to a portion of a string that might not be relevant
// if the string is destroyed but the index remains. you have to manage and synchronise two states.
// instead of doing that we just take a slice which is a reference to the string and doesn't need
// to be managed separately anymore.

fn main() {
    // create heap based immutable string
    let sentence = String::from("Hello World!");

    // pass in a reference to sentence to let first_word() "borrow" it
    // return a slice to the first word of the sentence
    let first_word = first_word(&sentence);

    println!("{}", first_word);
}


// return &str because a slice is a reference to a string (so we give it &)
// and it's a fixed size so sits on the stack easily (so its str not String)
fn first_word(s: &String) -> &str {
    // get a byte array of the &String to enumerate over
    let bytes = s.as_bytes();
    
    // .iter() returns an interator that enumerate() can use to return a tuple of index, value
    // much the same as pythons for i,v in enum(bytes)~
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return a reference to s, containing a slice up to the first space
            return &s[..i];
        }
    }
    // return a reference to s, containing the whole variable as it's only one word. (no space was found)
    &s[..]
}