// VARIABLES ONLY

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//         );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }



// WITH TUPLES

// this is more ordered, even though width vs height isn't clear
// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }



//WITH STRUCTS

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// WITH DERIVED TRAITS

#[derive(Debug)]                        // this allows us to explicitly opt-in to debugging print out
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);   // `:?` tells `println!` that we want to use an output format called `Debug`
                                        // which is better for printing our struct
                                        // we can also use `{:#?}`
}
