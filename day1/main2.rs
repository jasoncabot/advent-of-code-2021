use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), &'static str> {
    let file = File::open("input2.txt").expect("Make sure file exists");
    let reader = BufReader::new(file);

    let readings: Vec<i32> = reader
        .lines()
        .map(|l| l.expect("Can't read line").parse::<i32>().unwrap())
        .collect();

    let window_size: usize = 3;
    let mut idx_start = 0;
    let mut idx_end = 0 + (window_size - 1);

    if readings.len() < window_size {
        return Err("Input is too short, must be at least windowSize");
    }

    let mut sums = 0;
    let mut sum: i32 = readings[idx_start..=idx_end].iter().sum();
    println!("{} (N/A - no previous sum)", sum);

    while idx_end < readings.len() - 1 {
        let previous_sum = sum;

        // Adjust totals
        sum -= readings[idx_start];
        sum += readings[idx_end + 1];

        if sum > previous_sum {
            println!("{} (increased)", sum);
            sums += 1;
        } else if sum < previous_sum {
            println!("{} (decreased)", sum);
        } else {
            println!("{} (no change)", sum);
        }

        // Shift along
        idx_start += 1;
        idx_end += 1;
    }
    println!("In this example, there are {} sums that are larger than the previous sum.", sums);


    return Ok(());
}
