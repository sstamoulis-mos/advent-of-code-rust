advent_of_code::solution!(9);

use std::{fmt::Display, ops::Deref};

use ndarray::Array;

use DiskBlock::{File, Free};

enum DiskBlock {
    File(u8),
    Free,
}

struct DiskMap(Array<DiskBlock, ndarray::Ix1>);

impl Deref for DiskMap {
    type Target = Array<DiskBlock, ndarray::Ix1>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in self.iter() {
            match e {
                File(id) => write!(f, "{}", id)?,
                Free => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

impl DiskMap {
    fn from_input(input: &str) -> Self {
        let mut id = 0;
        Self(Array::from_iter(
            input
                .chars()
                .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
                .enumerate()
                .flat_map(|(i, e)| {
                    if i % 2 == 0 {
                        // free space
                        let file = (0..e).into_iter().map(|_| File(id)).collect::<Vec<_>>();
                        id += 1;
                        file
                    } else {
                        // file
                        (0..e).into_iter().map(|_| Free).collect::<Vec<_>>()
                    }
                }),
        ))
    }

    fn compact(&mut self) {
        
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = DiskMap::from_input(input);

    println!("{}", map);
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
