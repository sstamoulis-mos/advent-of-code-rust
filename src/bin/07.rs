use itertools::{repeat_n, Itertools};

advent_of_code::solution!(7);

#[derive(Debug)]
enum Operations {
    Add,
    Mul,
    Con,
}

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
    let operators = vec![Operations::Add, Operations::Mul];
    let mut sum = 0;

    for Equation { result, operands } in equations {
        // println!("{:?}: {:?}", result, operands);
        for op_list in repeat_n(operators.iter(), operands.len() - 1).multi_cartesian_product() {
            // println!("{:?}", op_list);
            let mut op_iter = op_list.iter();
            let r = operands.iter().copied().reduce(|acc, e| {
                let op = op_iter.next().unwrap();
                match op {
                    Operations::Add => acc + e,
                    Operations::Mul => acc * e,
                    Operations::Con => panic!("Cannot handle operand Concatenate"),
                }
            })?;
            if r == result {
                sum += result;
                break;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = Equation::from_input(input)?;
    let operators = vec![Operations::Add, Operations::Mul, Operations::Con];
    let mut sum = 0;

    for Equation { result, operands } in equations {
        // println!("{:?}: {:?}", result, operands);
        for op_list in repeat_n(operators.iter(), operands.len() - 1).multi_cartesian_product() {
            // println!("{:?}", op_list);
            let mut op_iter = op_list.iter();
            let r = operands.iter().copied().reduce(|acc, e| {
                let op = op_iter.next().unwrap();
                match op {
                    Operations::Add => acc + e,
                    Operations::Mul => acc * e,
                    Operations::Con => {
                        let digits = ((e as f64).log10() + 1.) as u32;
                        acc * 10u64.pow(digits) + e
                    }
                }
            })?;
            if r == result {
                sum += result;
                break;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
