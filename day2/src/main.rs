use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input2.txt").expect("Make sure file exists");
    let reader = BufReader::new(file);

    let mut position = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let command_line = line.unwrap();
        let commands: Vec<&str> = command_line.split_whitespace().collect();
        let amount = commands[1].parse::<i32>().unwrap();
        match commands[0] {
            "forward" => {
                position += amount;
                println!(
                    "forward {} adds {} to your horizontal position, a total of {}.",
                    amount, amount, position
                );
            }
            "down" => {
                depth += amount;
                println!(
                    "down {} adds {} to your depth, resulting in a value of {}.
                    ",
                    amount, amount, depth
                );
            }
            "up" => {
                depth -= amount;
                println!(
                    "up {} decreases your depth by {}, resulting in a value of {}",
                    amount, amount, depth
                );
            }
            _ => {
                panic!("Unknown command {}", commands[0]);
            }
        }
    }

    println!("After following these instructions, you would have a horizontal position of {} and a depth of {}. (Multiplying these together produces {}.)", 
        position, depth, position * depth);

    return Ok(());
}
