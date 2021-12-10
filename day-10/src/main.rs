use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let part1_score = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let part2_score = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let input = std::fs::read_to_string("input.txt")?;
    let parsed = input.lines().map(|line| {
        let stack = Vec::with_capacity(line.len());
        line.chars()
            .try_fold(stack, |mut stack, bracket| match bracket {
                '[' | '{' | '(' | '<' => {
                    stack.push(bracket);
                    Ok(stack)
                }
                _ => match (stack.pop().unwrap(), bracket) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => Ok(stack),
                    (_, invalid) => Err(invalid),
                },
            })
    });

    let (valid_lines, invalid_lines): (Vec<Vec<char>>, Vec<char>) = parsed.partition_result();

    let part1 = invalid_lines.iter().map(|x| part1_score[x]).sum::<u32>();

    let autocomplete_lines = valid_lines
        .into_iter()
        .map(|x| {
            x.iter()
                .rev()
                .fold(0u64, |score, c| score * 5 + part2_score[c])
        })
        .sorted()
        .collect_vec();
    let part2 = autocomplete_lines[autocomplete_lines.len() / 2];

    println!("{part1}");
    println!("{part2}");

    Ok(())
}
