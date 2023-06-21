use crate::project::output_setting::OutputSetting;
use crate::project::project_options::ProjectOptions;
use crate::project::input_setting::InputSetting;

fn work() {
    let options = ProjectOptions {
        input: InputSetting {
            directory: String::from("X:\\lhh\\project"),
        },
        output: OutputSetting {
            directory: String::from("X:\\lhh\\export"),
        },
    };
    println!("options: {:?}", options);

    let serialized = serde_json::to_string(&options).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: ProjectOptions = serde_json::from_str(serialized.as_str()).unwrap();
    println!("deserialized: {:?}", deserialized);
}

pub fn run() {
    work();
}
