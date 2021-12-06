use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn bit_counts(diagnostics: &[usize], bit_width: usize) -> Vec<usize> {
    // for every binary position in diagnostics
    // keep a tally for each 1 in that position
    let mut counts = vec![0; bit_width];
    for diagnostic in diagnostics {
        for i in 0..bit_width {
            let mask = 0b1 << (bit_width - 1) - i;
            counts[i] += (diagnostic & mask == mask) as usize;
        }
    }
    return counts;
}

fn calculate_rates(diagnostics: &[usize], bit_width: usize) -> (usize, usize) {
    let counts = bit_counts(&diagnostics, bit_width);
    let cutoff = (diagnostics.len() as f32 / 2.0).ceil() as usize;
   
    // Convert counts of 1's into a binary number
    let bits = counts_to_bits(&counts, cutoff);
    
    let mask = vec![1; bit_width]
        .iter()
        .enumerate()
        .fold(0, |total, (index, current)| total + (current << counts.len() - (1 + index)) );

    // gamma rate is count of 1's, epsilon is the inverse (being sure to mask to our length)
    // e.g:
    // 00001 flips to 1111111111111110 so mask to original diagnostic length 11110
    return (bits, (!bits) & mask);
}

fn counts_to_bits(counts: &[usize], cutoff: usize) -> usize {
    // convert an array of counts, e.g [7, 12, 0, 48, 0] into a binary representation
    // for if it's greater or less than the cutoff
    return counts
        .iter()
        .map(|c| (c >= &cutoff) as usize)
        .enumerate()
        .fold(0, |total, (index, current)| total + (current << counts.len() - (1 + index)) );
}

fn main() -> io::Result<()> {
    let file = File::open("input1.txt").expect("Make sure file exists");
    let reader = BufReader::new(file);

    let diagnostics: Vec<usize> = reader.lines()
        .map(|i| usize::from_str_radix(&i.unwrap(), 2).unwrap())
        .collect();

    let width = 12;

    let (gamma, epsilon) = calculate_rates(&diagnostics, width);

    println!("Gamma: '{:012b}' ({}), Epsilon '{:012b}' ({}) from {} diagnostics of length {}", gamma, gamma, epsilon, epsilon, diagnostics.len(), width);
    println!("= {}", gamma * epsilon);

    return Ok(());
}

#[test]
fn test_calculate_rates() {
    let diagnostics = vec![
        0b00100, 
        0b11110,
        0b10110,
        0b10111,
        0b10101,
        0b01111,
        0b00111,
        0b11100,
        0b10000,
        0b11001,
        0b00010,
        0b01010,
    ];

    let (gamma, epsilon) = calculate_rates(&diagnostics, 5);
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
}

#[test]
fn test_counts_to_bits() {
    let x = counts_to_bits(&[0, 5, 6, 7, 12], 6);
    assert_eq!(x, 0b00111);
}

#[test]
fn test_counts_to_bits2() {
    let x = counts_to_bits(&[5, 3, 8, 5, 4], 6);
    assert_eq!(x, 0b00100);
}

#[test]
fn test_calculate_rates2() {
    let diagnostics = vec![
        0b010001110001,
        0b110100000001,
        0b111001001011
    ];

    let (gamma, epsilon) = calculate_rates(&diagnostics, 12);
    assert_eq!(gamma, 0b110001000001);
    assert_eq!(epsilon, 0b001110111110);
}

#[test]
fn test_calculate_rates3() {
    let diagnostics = vec![
        0b010001110001,
        0b110100000001,
        0b111001001011,
        0b111100001000,
        0b000111101001,
        0b101111001001,
        0b111111000110,
        0b000101110010,
        0b001101001010,
        0b000110001101,
        0b011111111110,
        0b101100010001,
        0b110101100001,
        0b000000000110,
        0b010011000011
    ];
    let counts = bit_counts(&diagnostics, 12);
    assert_eq!(counts, [7, 8, 7, 11, 6, 10, 5, 4, 7, 4, 7, 9]);
    let bits = counts_to_bits(&counts, 8);
    assert_eq!(bits, 0b010101000001);
    let (gamma, epsilon) = calculate_rates(&diagnostics, 12);
    assert_eq!(gamma, 0b010101000001);
    assert_eq!(epsilon, 0b101010111110);
}

#[test]
fn test_counts() {
    let file = File::open("input1.txt").expect("Make sure file exists");
    let reader = BufReader::new(file);

    let diagnostics: Vec<usize> = reader.lines()
        .map(|i| usize::from_str_radix(&i.unwrap(), 2).unwrap())
        .collect();

    assert_eq!(diagnostics[0], 0b010001110001);

    let counts = bit_counts(&diagnostics, 12);

    assert_eq!(counts, vec![511, 498, 490, 520,470, 483, 503, 487,524, 491, 504, 492]);
}
