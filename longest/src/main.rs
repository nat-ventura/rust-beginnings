// the lifetime annotations in the fn signature
// mean that any values that do not adhere to
// this contract should be rejected by the
// borrow checker

// lifetime annotations always go in fn sig
// and not in the fn body!!!
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
    
fn main() {
    // we want string slices (references)
    // since we don't want the fn to take
    // ownership of its arguments

    // slice of a String
    let string1 = String::from("long string is long");

    // // string literal
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);
    
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
