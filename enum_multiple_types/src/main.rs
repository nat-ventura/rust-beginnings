fn main() {

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

// rust needs to know what types will be in the vector
// at compile time so it knows exactly how much
// memory on the heap will be needed to store each element
