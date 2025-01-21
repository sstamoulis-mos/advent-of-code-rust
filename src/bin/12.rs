advent_of_code::solution!(12);

mod part_one {
    pub struct Region {
        letter: char,
        positions: Vec<Position>,
    }

    impl Region {
        pub fn area(&self) -> u64 {
            todo!()
        }

        pub fn perimeter(&self) -> u64 {
            todo!()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Position {
        pub x: usize,
        pub y: usize,
    }

    pub struct Map {
        backing_array: ndarray::Array<char, ndarray::Ix2>,
    }

    #[derive(Debug)]
    pub struct ParseError;

    impl From<ndarray::ShapeError> for ParseError {
        fn from(_: ndarray::ShapeError) -> Self {
            Self
        }
    }

    impl TryFrom<&str> for Map {
        type Error = ParseError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let shape = (
                value.lines().count(),
                value.lines().next().unwrap().chars().count(),
            );
            let backing_array = ndarray::Array::from_shape_vec(
                shape,
                value.lines().flat_map(|line| line.chars()).collect(),
            )?;
            Ok(Map { backing_array })
        }
    }

    impl Map {
        pub fn get_regions(&self) -> Vec<Region> {
            let mut regions = Vec::new();
            let checked = ndarray::Array::from_elem(self.backing_array.raw_dim(), false);
            let mut possible_positions = vec![Position { x: 0, y: 0 }];
            let mut active_region = None;
            while (checked.iter().any(|&b| b)) {
                while(possible_positions.len()>0) {
                    
                }
            }
            regions
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    use part_one::Map;

    let map = Map::try_from(input).unwrap();
    let regions = map.get_regions();
    let total_price = regions
        .into_iter()
        .map(|r| (r.area(), r.perimeter()))
        .map(|(area, perimeter)| area * perimeter)
        .sum();
    Some(total_price)
}

pub fn part_two(_input: &str) -> Option<u64> {
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
