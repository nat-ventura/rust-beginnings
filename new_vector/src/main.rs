// fn main() {
    // let v = vec![1, 2, 3];
    // because we've given i32 values,
    // Rust can infer that the type of v is Vec<i32>,
    // and the type annotation isn't necessary...
    // let v: Vec<i32> = Vec::new();
    
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
// }

// like any other struct, a vector will be freed when it goes out of scope

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
    
//     // this gives us a reference
//     let third: &i32 = &v[2];

//     // this gives us an Option<&T>
//     let third: Option<&i32> = v.get(2);

//     // two ways to reference an element so you can choose
//     // how the program behaves when you try to use
//     // an index value that the vector doesn't have
//     // an element for

// }

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // use this method when you want a fatal error
    // let does_not_exist = &v[100];

    // use this if accessing an element beyond the range of the vector
    // happens occasionally under normal circumstances
    // i.e. if a user inputs a larger num than is present
    // you can then print all possible values to the user
    // and prompt them to input again instead of crashing the program
    // let does_not_exist = v.get(100);
   
    // immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // to change the value that the mutable reference refers to,
        // we have to use the dereference operator `*`
        // to get to the value in `i` before we can use the `+=` operator
        *i += 50;
    }
}
