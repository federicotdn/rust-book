// The function signature now tells Rust that for some lifetime 'a, the
// function takes two parameters, both of which are string slices that
// live at least as long as lifetime 'a. The function signature also
// tells Rust that the string slice returned from the function will live
// at least as long as lifetime 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Ultimately, lifetime syntax is about connecting the lifetimes of
// various parameters and return values of functions. Once they’re
// connected, Rust has enough information to allow memory-safe operations
// and disallow operations that would create dangling pointers or
// otherwise violate memory safety.


// Does not work, as string2 has a shorter lifetime!

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }
