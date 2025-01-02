use std::fmt::Debug;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn from_input(input: &str) -> Option<Vec<Equation>> {
        input
            .lines()
            .map(|line| {
                let mut it = line.split(": ");
                let result = it.next()?.parse().ok()?;
                let operands = it
                    .next()?
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().ok())
                    .collect::<Option<Vec<u64>>>()?;
                Some(Equation { result, operands })
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = Equation::from_input(input)?;
    let operators = vec![u64::checked_add, u64::checked_mul];
    // println!("{:#?}", equations);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
