use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let file_path_arg = match env::args().nth(1) {
        Some(str) => str,
        None => panic!("Specify input file path"),
    };

    let current_dir = env::current_dir().unwrap();
    let file_path = Path::join(&current_dir, file_path_arg);
    let contents = fs::read_to_string(file_path).unwrap();

    let mut accumulator = String::new();
    let mut last_value = -1;
    let mut increase_count = 0;

    for char in contents.chars() {
        let value = match char {
            '\n' | '\r' => i32::from_str_radix(accumulator.as_str(), 10).unwrap(),
            _ => {
                accumulator.push(char);
                continue;
            }
        };

        accumulator.clear();

        increase_count += match last_value {
            -1 => 0,
            _ if value > last_value => 1,
            _ => 0,
        };

        last_value = value;
    }

    println!("Increased {} times", increase_count);
}
