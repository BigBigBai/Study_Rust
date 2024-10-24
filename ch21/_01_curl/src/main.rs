use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

fn main() -> Result<()> {
    // Example of a Person instance
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    // Serialize the `Person` struct into a JSON string
    let serialized = serde_json::to_string(&person)?;
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back into a `Person` struct
    let deserialized: Person = serde_json::from_str(&serialized)?;
    println!("Deserialized: {:?}", deserialized);

    // Working with raw JSON data using `Value`
    let raw_data = r#"
    {
        "name": "Bob",
        "age": 25,
        "email": "bob@example.com"
    }
    "#;
    let parsed: Value = serde_json::from_str(raw_data)?;
    println!("Name from raw JSON: {}", parsed["name"]);

    Ok(())
}
