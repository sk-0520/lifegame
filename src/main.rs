extern crate serde_json;

mod project_options;

fn main() {
    let options = project_options::ProjectOptions {
        project_directory: String::from("X:\\lhp\\project"),
        output_directory: String::from("X:\\lhp\\export"),
    };
    println!("options: {:?}", options);

    let serialized = serde_json::to_string(&options).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: project_options::ProjectOptions = serde_json::from_str(serialized.as_str()).unwrap();
    println!("deserialized: {:?}", deserialized);

}
