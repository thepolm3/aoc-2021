
use itertools::Itertools;

fn main() {
    let nums: Vec<u32> = include_str!("input.txt").lines().map(|x| x.parse().unwrap()).collect();

    println!("{}",
        nums.iter()
            .tuple_windows().map(|(x,y)| (x<y) as u32)
            .sum::<u32>()
    );

    println!("{}",
        nums.into_iter()
            .tuple_windows().map(|(x, _, _, y)| (x<y) as u32)
            .sum::<u32>()
    );
}
