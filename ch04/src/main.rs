// Ownership rules:
// - Each value in Rust has a variable that’s called its owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

// Reference rules:
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.


fn main() {
    println!("Hello, world!");

    // String will be destroyed when variable goes out of scope
    // Similar to RAII in C++
    let _s = String::from("hello");

    // Value is moved to _s2 (not copied)
    let _s2 = _s;

    // Would not work, since value was moved out of _s
    // println!("_s is: {}", _s)

    // Copy value
    let _s3 = _s2.clone();
    println!("_s3: {}", _s3);

    takes_ownership(_s3);

    // Would not work
    // println!("_s3: {}", _s3);

    let _s4 = gives_ownership();
    println!("Got: {}", _s4);

    // &_s4: create reference to _s4
    println!("len _s4: {}", calculate_length(&_s4));

    // The type of _s5 is &str (string slice, inmutable)
    let _s5 = "Hello, world!";

    // The type of _s6 is an inmutable string slice as well
    let _s6 = first_word_v2(&_s4);
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn change_string(s: &mut String) {
    // Receive s by mutable reference
    s.push_str("...");
}

fn calculate_length(s: &String) -> usize {
    // Receive s by reference (borrowing)

    // Would not work; can't modify borrowed values
    // s.push_str("...");
    
    s.len()
}

fn takes_ownership(s: String) {
    // Received s by move
    println!("Received: {}", s);
    // s is destroyed
}

fn gives_ownership() -> String {
    let s = String::from("hellooo");
    s
}
