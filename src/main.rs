mod setting;

use setting::Setting;

fn main() {
    let setting = Setting {
        width: 16,
        height: 16,
        outer: setting::Outer::Reverse,
    };
    println!("{:?}", setting);
}
