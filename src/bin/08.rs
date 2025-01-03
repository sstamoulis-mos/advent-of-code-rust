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
    // let (rows, columns) = map.raw_dim().into_pattern();
    let antenna_locations = map.get_antenna_locations();
    // println!("{:#?}", antenna_locations);
    let mut antinode_map = Array::from_elem(map.raw_dim(), false);
    for (_antenna_type, locations) in antenna_locations {
        // println!("{}", _antenna_type);
        for pair in locations
            .iter()
            .map(|l| (isize::try_from(l.0).unwrap(), isize::try_from(l.1).unwrap()))
            .permutations(2)
        {
            let &[l1, l2] = pair.as_slice() else {
                panic!("Too many elements")
            };
            // println!("{:?} - {:?}", l1, l2);
            let distance = (l1.0 - l2.0, l1.1 - l2.1);
            let antinode_locations = [
                (l1.0 + distance.0, l1.1 + distance.1),
                (l2.0 - distance.0, l2.1 - distance.1),
            ];
            for loc in antinode_locations {
                if let (Ok(r), Ok(c)) = (usize::try_from(loc.0), usize::try_from(loc.1)) {
                    if let Some(a) = antinode_map.get_mut((r, c)) {
                        *a = true;
                    }
                }
            }
        }
    }
    // println!("{:?}", antinode_map);
    let count = antinode_map.fold(0, |mut acc, &e| {
        if e {
            acc += 1;
        }
        acc
    });
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from_input(input)?;
    // let (rows, columns) = map.raw_dim().into_pattern();
    let antenna_locations = map.get_antenna_locations();
    // println!("{:#?}", antenna_locations);
    let mut antinode_map = Array::from_elem(map.raw_dim(), false);
    for (_antenna_type, locations) in antenna_locations {
        // println!("{}", _antenna_type);
        for pair in locations
            .iter()
            .map(|l| (isize::try_from(l.0).unwrap(), isize::try_from(l.1).unwrap()))
            .permutations(2)
        {
            let &[l1, l2] = pair.as_slice() else {
                panic!("Too many elements")
            };
            // println!("{:?} - {:?}", l1, l2);
            let distance = (l1.0 - l2.0, l1.1 - l2.1);
            for locations in (0..).map(|i| {
                [
                    (l1.0 + i * distance.0, l1.1 + i * distance.1),
                    (l2.0 - i * distance.0, l2.1 - i * distance.1),
                ]
            }) {
                let mut found = false;
                for loc in locations.into_iter().filter_map(|(r, c)| {
                    match (usize::try_from(r), usize::try_from(c)) {
                        (Ok(r), Ok(c)) => Some((r, c)),
                        _ => None,
                    }
                }) {
                    if let Some(antinode) = antinode_map.get_mut(loc) {
                        *antinode = true;
                        found = true;
                    }
                }
                if !found {
                    break;
                }
            }
        }
    }
    // println!("{:?}", antinode_map);
    let count = antinode_map.fold(0, |mut acc, &e| {
        if e {
            acc += 1;
        }
        acc
    });
    Some(count)
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
        assert_eq!(result, Some(34));
    }
}
