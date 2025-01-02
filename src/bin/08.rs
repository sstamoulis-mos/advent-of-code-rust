advent_of_code::solution!(8);

use std::{collections::HashMap, ops::Deref};

use itertools::Itertools;
use ndarray::Array;

type Location = (usize, usize);

struct Map {
    array: Array<char, ndarray::Ix2>,
}

impl Deref for Map {
    type Target = Array<char, ndarray::Ix2>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl Map {
    fn from_input(input: &str) -> Option<Self> {
        let rows = input.lines().count();
        let columns = input.lines().next()?.chars().count();
        let array = Array::from_shape_vec(
            (rows, columns),
            input.lines().flat_map(|line| line.chars()).collect(),
        )
        .ok()?;
        Some(Map { array })
    }

    fn get_antenna_locations(&self) -> HashMap<char, Vec<Location>> {
        self.indexed_iter()
            .filter(|(_, &e)| e != '.')
            .fold(HashMap::new(), |mut acc, (loc, &e)| {
                acc.entry(e).or_default().push(loc);
                acc
            })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_input(input)?;
    let antenna_locations = map.get_antenna_locations();
    // println!("{:#?}", antenna_locations);
    let mut antinode_map = Array::from_elem(map.raw_dim(), false);
    for (_antenna_type, locations) in antenna_locations {
        for pair in locations.iter().combinations(2) {
            let &[l1, l2] = pair.as_slice() else {
                panic!("Too many elements")
            };
            println!("{:?} - {:?}", l1, l2);
            let a = (l2.1 - l1.1) / (l2.0 - l1.0);
            let b = l1.1 - a * l1.0;
            println!("y = {}x + {}", a, b);
        }
    }
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
