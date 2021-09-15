use serde_json::{json, Number, Value};

#[test]
fn value_array() {
    println!("Array {}", json!([1,2,3]))
}