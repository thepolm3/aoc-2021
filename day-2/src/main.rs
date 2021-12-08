use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let part1 = std::fs::read_to_string("input.txt")?
    .lines().map(|x| {
        let x = x.split_whitespace().collect::<Vec<_>>();
        let (direction, value) = (x[0],x[1].parse::<i32>().unwrap());

        match direction {
            "forward" => (value, 0),
            "up" => (0, -value),
            "down" => (0, value),
            _ => (0, 0),
        }
    }).fold((0,0),|(x,y),(z,w)| (x+z,y+w));

    println!("{}", part1.0*part1.1);

    let part2 = std::fs::read_to_string("input.txt")?
        .lines().fold((0,0,0),|(pos,depth, aim), s| {
            let s_parts = s.split_whitespace().collect::<Vec<_>>();
            let (direction, value) = (s_parts[0],s_parts[1].parse::<i32>().unwrap());

            match direction {
                "forward" => (pos+value, depth+value*aim, aim),
                "up" => (pos, depth, aim-value),
                "down" => (pos, depth, aim+value),
                _ => (pos, depth, aim),
            }
        });

    println!("{}", part2.0*part2.1);

    
    Ok(())
}
