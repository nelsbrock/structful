use super as structful;
use super::*;
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use structful_derive::{StructfulGet, StructfulPut};

#[derive(StructfulGet, StructfulPut, Serialize, Deserialize)]
struct Wow(u8, u8);

#[derive(Serialize, Deserialize, StructfulGet, StructfulPut)]
struct Person {
    name: String,
    age: u8,
    #[structful(leaf)]
    aha: u16,
    wow: Wow,
}

#[test]
fn test() {
    let person = Person {
        name: "Niklas".into(),
        age: 24,
        aha: 3,
        wow: Wow(45, 67),
    };

    person
        .structful_get(
            "wow/1".split('/'),
            &mut serde_json::Serializer::pretty(std::io::stdout()),
        )
        .unwrap();
}
