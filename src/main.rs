extern crate serde_json;

mod project_options;

use project_options::project_options::ProjectOptions;

fn main() {
    let options = ProjectOptions {
        project_directory: String::from("X:\\lhp\\project"),
        output_directory: String::from("X:\\lhp\\export"),
    };
    println!("options: {:?}", options);

    let serialized = serde_json::to_string(&options).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: ProjectOptions = serde_json::from_str(serialized.as_str()).unwrap();
    println!("deserialized: {:?}", deserialized);

}
