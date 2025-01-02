use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

advent_of_code::solution!(5);

struct Rules<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'a> Deref for Rules<'a> {
    type Target = HashMap<&'a str, HashSet<&'a str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rules<'_> {
    fn is_correct(&self, update: &[&str]) -> bool {
        update.iter().enumerate().all(|(i, k)| {
            if let (Some(after), Some(set)) = (update.get(i + 1..), self.get(k)) {
                !after.iter().any(|&v| set.contains(v))
            } else {
                true
            }
        })
    }

    fn fix<'a>(&self, update: &'a [&str]) -> Vec<&'a str> {
        let mut fixed = Vec::from(update);
        let mut i = 0;
        while i < fixed.len() {
            if let (Some(after), Some(set)) = (fixed.get(i + 1..), self.get(fixed[i])) {
                if let Some(j) = after
                    .iter()
                    .enumerate()
                    .find_map(|(j, &v)| set.contains(v).then_some(j))
                {
                    fixed.swap(i, j + i + 1);
                    continue;
                }
            }
            i += 1;
        }
        fixed
    }
}

struct Data<'a> {
    rules: Rules<'a>,
    updates: Vec<Vec<&'a str>>,
}

fn parse_input(input: &str) -> Data {
    let rules = Rules(
        input
            .lines()
            .filter(|l| l.contains('|'))
            .filter_map(|l| match l.split('|').collect::<Vec<_>>().as_slice() {
                &[left, right] => Some((right, left)),
                _ => None,
            })
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_default().insert(v);
                acc
            }),
    );
    // println!("{:#?}", rules);
    let updates = input
        .lines()
        .filter(|l| l.contains(','))
        .map(|l| l.split(',').collect::<Vec<_>>())
        .collect();
    Data { rules, updates }
}

pub fn part_one(input: &str) -> Option<u64> {
    let Data { rules, updates } = parse_input(input);
    let sum = updates
        .iter()
        .filter(|update| rules.is_correct(update))
        // .inspect(|u| println!("{:#?}", u))
        .map(|u| u.get(u.len() / 2).and_then(|s| s.parse::<u64>().ok()))
        .sum();
    sum
}

pub fn part_two(input: &str) -> Option<u64> {
    let Data { rules, updates } = parse_input(input);
    let sum = updates
        .iter()
        .filter(|update| !rules.is_correct(update))
        // .inspect(|u| println!("{:#?}", u))
        .map(|update| rules.fix(update))
        // .inspect(|u| println!("{:#?}", u))
        .map(|u| u.get(u.len() / 2).and_then(|s| s.parse::<u64>().ok()))
        .sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
