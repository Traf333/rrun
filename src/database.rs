use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    timezone: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct PersonList {
    persons: Vec<Person>,
}


pub fn read_data() {
    let data = {
        // Load the first file into a string.
        let text = std::fs::read_to_string("data.json").unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<PersonList>(&text).unwrap()
    };

    for person in data.persons.iter() {
        println!("Name: {}, timezone: {}", person.name, person.timezone);
    }
}

fn write_data(data: PersonList) {}
