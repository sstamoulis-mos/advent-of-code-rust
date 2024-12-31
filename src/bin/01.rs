use std::{collections::HashMap, ops::Mul};

advent_of_code::solution!(1);

fn read_to_lists(input: &str) -> Option<(Vec<u64>, Vec<u64>)> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut numbers = line.split_ascii_whitespace();
        list1.push(numbers.next().and_then(|s| s.parse().ok())?);
        list2.push(numbers.next().and_then(|s| s.parse().ok())?);
    }
    Some((list1, list2))
}

pub fn part_one(input: &str) -> Option<u64> {
    if let Some((mut list1, mut list2)) = read_to_lists(input) {
        list1.sort();
        list2.sort();
        let diff = list1
            .iter()
            .zip(list2.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum();
        Some(diff)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Some((list1, list2)) = read_to_lists(input) {
        let map = list1.iter().fold(HashMap::<u64, u64>::new(), |mut acc, x| {
            if !acc.contains_key(x) {
                let count = list2.iter().filter(|v| **v == *x).count() as u64;
                acc.insert(*x, count);
            }
            acc
        });
        let result = list1
            .iter()
            .map(|x| map.get(x).and_then(|v| Some(x.mul(*v))))
            .sum();
        result
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
