use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // we aren't able to use variables field_name and field_value
    // after they've been moved into the hash map with the call to insert
    // if you make an insert statement twice, the initial value will be replaced
}

// for types that implement the `Copy` trait like i32,
// the values are copied into the hash map
// for owned values like `String`, the values will be moved
// and the hashmap will be the owner of those values
