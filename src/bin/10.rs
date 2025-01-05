use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

use ndarray::Array;

advent_of_code::solution!(10);

struct Map(Array<u8, ndarray::Ix2>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for Map {
    type Target = Array<u8, ndarray::Ix2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Map {
    pub fn from_input(input: &str) -> Option<Self> {
        let shape = (
            input.lines().count(),
            input.lines().next().unwrap().chars().count(),
        );
        Some(Self(
            Array::from_shape_vec(
                shape,
                input
                    .lines()
                    .flat_map(|line| {
                        line.chars()
                            .map(|c| c.to_digit(10))
                            .filter_map(|o| o.and_then(|d| u8::try_from(d).ok()))
                    })
                    .collect(),
            )
            .ok()?,
        ))
    }

    pub fn trailheads(&self) -> Vec<(usize, usize)> {
        self.indexed_iter()
            .filter_map(|(i, e)| (*e == 0).then_some(i))
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_input(input)?;
    // println!("{}", map);
    let trailheads = map.trailheads();
    let mut visited: HashMap<_, _> = trailheads.iter().map(|&i| (i, HashSet::new())).collect();
    let mut stack: Vec<_> = trailheads.into_iter().map(|i| (i, i)).collect();
    let mut count = 0;
    while let Some((root, index)) = stack.pop() {
        let value = map[index];
        let (row, column) = index;
        let candidates = [
            row.checked_sub(1).and_then(|r| {
                let index = (r, column);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
            row.checked_add(1).and_then(|r| {
                let index = (r, column);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
            column.checked_sub(1).and_then(|c| {
                let index = (row, c);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
            column.checked_add(1).and_then(|c| {
                let index = (row, c);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
        ];
        for candidate in candidates {
            if let Some((c_index, c_value)) = candidate {
                if c_value == value + 1 {
                    if c_value == 9 {
                        if let Some(root_visited) = visited.get_mut(&root) {
                            if !root_visited.contains(&c_index) {
                                count += 1;
                                root_visited.insert(c_index);
                            }
                        }
                    } else {
                        stack.push((root, c_index));
                    }
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from_input(input)?;
    // println!("{}", map);
    let trailheads = map.trailheads();
    let mut stack: Vec<_> = trailheads;
    let mut count = 0;
    while let Some(index) = stack.pop() {
        let value = map[index];
        let (row, column) = index;
        let candidates = [
            row.checked_sub(1).and_then(|r| {
                let index = (r, column);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
            row.checked_add(1).and_then(|r| {
                let index = (r, column);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
            column.checked_sub(1).and_then(|c| {
                let index = (row, c);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
            column.checked_add(1).and_then(|c| {
                let index = (row, c);
                map.get(index).and_then(|&v| Some((index, v)))
            }),
        ];
        for candidate in candidates {
            if let Some((c_index, c_value)) = candidate {
                if c_value == value + 1 {
                    if c_value == 9 {
                        count += 1;
                    } else {
                        stack.push(c_index);
                    }
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
