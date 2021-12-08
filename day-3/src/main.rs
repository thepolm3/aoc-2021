use std::cmp::Ordering;
use itertools::Itertools;

enum SlidingFilter {
    MostSignificant,
    LeastSignificant
}
fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            u32::from_str_radix(line, 2).unwrap()
        }).collect_vec();
    let bit_length = 12;
    println!("{}", part1(&lines, bit_length));
    println!("{}", part2(&lines, bit_length));
}

fn get_bit(n: u32, i: u32) -> u32 {
    (n & 1 << i) >> i
}

fn sliding_bit_filter(mut values: Vec<u32>, bit_length: u32, significance: SlidingFilter) -> u32 {
    for i in (1..=bit_length).map(|x| bit_length - x) {
        let (ones, zeros): (Vec<u32>, Vec<u32>) = values.iter().partition(|&&rating| get_bit(rating, i) == 0);
        values = 
        match (&significance, (ones.len().cmp(&(values.len()/2)))) {
            (SlidingFilter::MostSignificant, Ordering::Greater) |
            (SlidingFilter::MostSignificant, Ordering::Equal) |
            (SlidingFilter::LeastSignificant, Ordering::Less) => ones,

            (SlidingFilter::MostSignificant, Ordering::Less) |
            (SlidingFilter::LeastSignificant, Ordering::Greater) |
            (SlidingFilter::LeastSignificant, Ordering::Equal) => zeros,
        };
        if values.len() == 1 {break}
    }
    values[0]
}

fn part2(lines: &[u32], bit_length: u32) -> u32 {
    sliding_bit_filter(lines.to_owned(), bit_length, SlidingFilter::MostSignificant) *
    sliding_bit_filter(lines.to_owned(), bit_length, SlidingFilter::LeastSignificant)
}

fn part1(lines: &[u32], bit_length: u32) -> u32 {

    let total = lines.len() as u32;
    let mut gamma_rate = 0;
    for i in 0..bit_length {
        let ones: u32 = lines.iter().map(|&x| get_bit(x,i)).sum::<u32>();
        gamma_rate += ((ones > total/2) as u32) << i;
    }
    //zero out all of the bits except the last 12
    let epsilon_rate = !gamma_rate & ((1u32<<bit_length) - 1u32);
    gamma_rate*epsilon_rate

}
