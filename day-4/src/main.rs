#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use itertools::Itertools;
#[derive(Debug)]
struct IncorrectSizeVecError ();

// Bingo card
// when a list of (unique) numbers is drawn (using .draw)
// keeps track of if (and where) it's received those numbers
#[derive(Debug, Clone)]
struct BingoCard<const N: usize>
where [(); N*N]: Sized,
{
    board: [u32; N*N],
    xwins: [u8; N],
    ywins: [u8; N],
    hitsum: u32,
}

impl<const N: usize> std::fmt::Display for BingoCard<N> 
where [u32; N*N]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 
            self.board.chunks(N).into_iter().map(|x|
                x.iter().map(|z|
                    format!("{:02}", z)
                ).join(", ")
            ).join("\n")
        )
    }
}

impl<const N: usize> TryFrom<Vec<u32>> for BingoCard<N>
where [(); N*N]: Sized,
{
    type Error = IncorrectSizeVecError;

    fn try_from(vec: Vec<u32>) -> Result<Self, Self::Error> {
        if vec.len() != N*N{
            Err(IncorrectSizeVecError())
        } else {
            Ok(BingoCard::<N>::from_vec_unchecked(vec))
        }

    }
}

impl<const N: usize> BingoCard<N>
where [(); N*N]: Sized,
{
    //panics if the vec does not have length N*N
    fn from_vec_unchecked(from: Vec<u32>) -> Self {
        BingoCard {
            board: from.try_into().unwrap(),
            xwins: [0; N],
            ywins: [0; N],
            hitsum: 0,
        }
    }

    fn get_coords(index: usize) -> (usize, usize) {
        ((index % N), (index / N))
    }

    //draw card assumes that this number has never been drawn before
    //which allows certain optimisations
    fn draw_card(&mut self, number: u32) {
        if let Some(i) = self.board.iter().position(|&x| x == number) {
            let (x, y) = BingoCard::<N>::get_coords(i);
            self.xwins[y] += 1;
            self.ywins[x] += 1;
            self.hitsum+=number;
        }
    }

    fn has_won(&self) -> bool {
        return self
            .xwins
            .iter()
            .chain(self.ywins.iter())
            .any(|&x| x == 5);
    }

    fn sum_unmarked_numbers(&self) -> u32 {
        self.board.iter().sum::<u32>() - self.hitsum
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let sequence = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());

    let cards: Vec<BingoCard<6>> = input
        .lines()
        .skip(2)
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            chunk
                .flat_map(|line| 
                    line
                    .split_whitespace()
                    .map(|x|
                        x.parse::<u32>().unwrap()
                    )
                )
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec();
    
    println!("{}", cards[5]);

    assert_eq!(part1(cards.clone(), sequence.clone()),Some(38913));
    assert_eq!(part2(cards, sequence),Some(16836))
}

fn part2<const N: usize>(mut cards: Vec<BingoCard<N>>, sequence: impl Iterator<Item=u32>) -> Option<u32>
where [u32; N*N]: Sized,
{
    for number in sequence {
        cards = cards.into_iter().filter(|card| !card.has_won()).collect();
        for card in &mut cards {
            card.draw_card(number);
        }
        if cards.len() == 1 && cards[0].has_won() {
            return Some(cards[0].sum_unmarked_numbers()*number)
        }
    }
    None
}
fn part1<const N: usize>(mut cards: Vec<BingoCard<N>>, sequence: impl Iterator<Item=u32>) -> Option<u32>
where [u32; N*N]: Sized, 
{
    for number in sequence {
        for card in &mut cards {
            card.draw_card(number);
        }
        cards.iter().all(|x| x.has_won());

        if let Some(card) = cards.iter().find(|card| card.has_won()) {
            return Some(card.sum_unmarked_numbers() * number)
        }
    }
    None
}
