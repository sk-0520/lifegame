use crate::project::input_setting::InputSetting;
use crate::project::output_setting::OutputSetting;
use crate::project::project_options::ProjectOptions;

fn get_project_options() -> ProjectOptions {
    let options = ProjectOptions {
        input: InputSetting {
            directory: String::from("X:\\lhh\\project"),
        },
        output: OutputSetting {
            directory: String::from("X:\\lhh\\export"),
        },
    };

    let serialized = serde_json::to_string(&options).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: ProjectOptions = serde_json::from_str(serialized.as_str()).unwrap();
    println!("deserialized: {:?}", deserialized);

    return options;
}

fn work() {
    let options = get_project_options();
    
    println!("options: {:?}", options);
}

pub fn run() {
    work();
}
