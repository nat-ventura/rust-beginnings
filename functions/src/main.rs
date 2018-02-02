fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);

    lil_loop();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn lil_loop() {
    for number in (1..10).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}
