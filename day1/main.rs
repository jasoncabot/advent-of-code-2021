use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input2.txt").expect("Make sure file exists");
    let reader = BufReader::new(file);

    let mut current: Option<i32> = Option::None;
    let mut measurements = 0;
    for line in reader.lines() {
        let depth: i32 = line.expect("invalid int").parse::<i32>().unwrap();
        match current {
            Some(current) if depth > current => {
                println!("{} (increased)", depth);
                measurements += 1;
            }
            Some(current) if depth < current => println!("{} (decreased)", depth),
            Some(_) => println!("{} (N/A)", depth),
            None => println!("{} (N/A - no previous measurement)", depth),
        }
        current = Some(depth);
    }

    println!(
        "In this example, there are {} measurements that are larger than the previous measurement.",
        measurements
    );

    return Ok(());
}
