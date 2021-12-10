use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct SeaFloor<const N: usize, const M: usize> {
    grid: Vec<u32>,
}

impl<const N: usize, const M: usize> SeaFloor<N, M> {
    fn from_vec(vec: Vec<u32>) -> Self {
        assert!(vec.len() == N * M);
        Self { grid: vec }
    }
    fn get(&self, x: usize, y: usize) -> u32 {
        self.grid[N * y + x]
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        [
            if x != 0 { Some((x - 1, y)) } else { None },
            if y != 0 { Some((x, y - 1)) } else { None },
            if x < N - 1 { Some((x + 1, y)) } else { None },
            if y < M - 1 { Some((x, y + 1)) } else { None },
        ]
        .into_iter()
        .filter_map(|x| x)
    }

    fn is_minimum(&self, x: usize, y: usize) -> bool {
        let value = self.get(x, y);
        self.neighbors(x, y)
            .into_iter()
            .map(|(x, y)| self.get(x, y))
            .all(|nbr_value| nbr_value > value)
    }

    fn basin(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut basin = HashSet::from([(x, y)]);
        loop {
            let neighbors: HashSet<(usize, usize)> = basin
                .iter()
                .flat_map(|(x, y)| self.neighbors(*x, *y))
                .unique()
                .filter(|(x, y)| self.get(*x, *y) != 9)
                .collect();
            if neighbors.difference(&basin).count() == 0 {
                break basin;
            }
            basin.extend(neighbors);
        }
    }
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let sea_floor: Vec<u32> = input
        .lines()
        .flat_map(|line| line.chars().map(|char| char.to_string().parse::<u32>()))
        .try_collect()?;

    let sea_floor = SeaFloor::<100, 100>::from_vec(sea_floor);

    let mins = (0..10000usize)
        .into_iter()
        .map(|pos| (pos % 100, pos / 100))
        .filter(|(x, y)| sea_floor.is_minimum(*x, *y))
        .collect_vec();

    let part1 = mins
        .iter()
        .map(|(x, y)| sea_floor.get(*x, *y) + 1)
        .sum::<u32>();

    let part2 = mins
        .iter()
        .map(|(x, y)| sea_floor.basin(*x, *y).len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();

    println!("{part1}");
    println!("{part2}");
    
    Ok(())
}
