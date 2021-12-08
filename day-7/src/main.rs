fn triangular_sum(n: u32) -> u32 {
    (n * (n+1)) / 2
}

fn fuel_spent_linear(n:u32, values: &[u32]) -> u32 {
    values.iter().map(|&x| (x as i32 - n as i32).abs()).sum::<i32>() as u32
}

fn fuel_spent_triangular(n: u32, values: &[u32]) -> u32 {
    values.iter().map(|&x| triangular_sum((x as i32 - n as i32).abs() as u32)).sum::<u32>()
}

fn part1(values: &[u32]) -> u32 {
    let median = match values.len() % 2 {
        0 => values[(values.len() / 2)],
        _ => ((values[(values.len() / 2)] + values[((values.len()+1) / 2)]))/2,
    };
    fuel_spent_linear(median, &values).min(fuel_spent_linear(median+1, &values))
}

fn part2(values: &[u32]) -> u32 {
    let mean = values.iter().sum::<u32>()/values.len() as u32;
    fuel_spent_triangular(mean, values).min(fuel_spent_triangular(mean, values))
}
fn main() {
    let mut values: Vec<u32> = std::fs::read_to_string("input.txt").unwrap().split(',').map(|x| x.parse().unwrap()).collect();
    values.sort();
    
    println!("part1: {}", part1(&values));
    println!("part2: {}", part2(&values));
     
}
