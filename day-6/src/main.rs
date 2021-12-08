use num_bigint::BigUint;
use num_traits::{Zero, One};

#[derive(Debug, Clone, Eq, PartialEq)]
struct LanternFishPopulation {
    population: Vec<BigUint>,
}

impl LanternFishPopulation {
    //panics if the iterator has any values
    fn from_iter(iter: impl Iterator<Item = usize>) -> Self {
        let mut population: Vec<BigUint> = vec![Zero::zero(); 9];
        for item in iter {
            population[item] += 1u8;
        }
        Self { population }
    }

    fn advance_day(&mut self) {
        let breeders = self.population[0].clone();
        self.population.rotate_left(1);
        self.population[6] += breeders.clone();
        self.population[8] = breeders;
    }

    fn advance_week(&mut self) {
        let mut children = self.population.clone();
        children.rotate_right(2);
        self.population[7] = Zero::zero();
        self.population[8] = Zero::zero();
        for i in 0..9 {
            self.population[i] += children[i].clone();
        }
    }

    fn advance_by(&mut self, t: usize) {
        assert_eq!(7 * (t / 7) + t % 7, t);
        for _ in 0..(t / 7) {
            self.advance_week();
        }
        for _ in 0..(t % 7) {
            self.advance_day();
        }
    }

    fn total(&self) -> BigUint {
        self.population.iter().sum()
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut population = LanternFishPopulation::from_iter(
        std::fs::read_to_string("input.txt")?
            .split(",")
            .map(|x| x.parse::<usize>().unwrap()),
    );
    println!("{population:?}");
    population.advance_by(80);
    // println!("{population:?}");
    println!("{:?}", population.total());
    population.advance_by(256 - 80);
    // println!("{population:?}");
    println!("{:?}", population.total());

    Ok(())
}
