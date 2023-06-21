extern crate serde_json;

mod project;

use project::output_setting::OutputSetting;
use project::project_options::ProjectOptions;
use project::input_setting::InputSetting;

fn main() {
    let options = ProjectOptions {
        input: InputSetting {
            directory: String::from("X:\\lhp\\project"),
        },
        output: OutputSetting {
            directory: String::from("X:\\lhp\\export"),
        },
    };
    println!("options: {:?}", options);

    let serialized = serde_json::to_string(&options).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: ProjectOptions = serde_json::from_str(serialized.as_str()).unwrap();
    println!("deserialized: {:?}", deserialized);
}
