use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
const SEGMENTS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn get_key(unique_digits: &[&str]) -> HashMap<char, char> {
    let mut result = HashMap::new();
    let mut letter_count: HashMap<char, u32> = HashMap::new();
    let &four = unique_digits
        .iter()
        .find(|&&x| x.len() == 4)
        .unwrap();
    for letter in unique_digits.iter().flat_map(|str| str.chars()) {
        let entry = letter_count.entry(letter).or_default();
        *entry += 1
    }
    for (letter, freq) in letter_count.iter() {
        result.insert(
            letter.to_owned(),
            match freq {
                4 => 'e',
                6 => 'b',
                7 => {
                    if four.chars().contains(letter) {
                        'd'
                    } else {
                        'g'
                    }
                }
                8 => {
                    if four.chars().contains(letter) {
                        'c'
                    } else {
                        'a'
                    }
                }
                9 => 'f',
                _ => panic!("Invalid `unique_digits`"),
            },
        );
    }

    result
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let displays: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .flat_map(|line| {
            line.split('|')
                .map(|part| part.split_whitespace().collect_vec())
                .collect_tuple()
        })
        .collect();

    let part1 = displays
        .iter()
        .flat_map(|(_, b)| b)
        .map(|x| x.len())
        .filter(|x| [2, 3, 4, 7].contains(x))
        .count();
    println!("{}", part1);

    let part2 = displays
        .into_iter()
        .map(|(unique_digits, output)| {
            let key = get_key(&unique_digits);
            output
                .into_iter()
                .map(move |encoded_string| {
                    let decoded: String =
                        encoded_string.chars().map(|c| key[&c]).sorted().collect();
                    format!("{}", SEGMENTS.iter().position(|&x| x == decoded).unwrap())
                })
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();

    println!("{}", part2);

    Ok(())
}
