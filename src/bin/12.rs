advent_of_code::solution!(12);

mod part_one {
    use std::array;

    use ndarray::IntoDimension;

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

    impl ndarray::IntoDimension for Position {
        type Dim = ndarray::Ix2;

        fn into_dimension(self) -> Self::Dim {
            [self.y, self.x].into_dimension()
        }
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
            let mut active_region = Region {
                letter: todo!(),
                positions: todo!(),
            };
            while (checked.iter().any(|&b| b)) {
                while let Some(pos) = possible_positions.pop() {
                    if self.backing_array[pos.into_dimension()] == active_region.letter {
                        active_region.positions.push(pos);
                        for possible_position in self.get_adjacent_positions(pos) {
                            possible_positions.push(possible_position);
                        }
                    }
                }
            }
            regions
        }

        fn get_adjacent_positions(&self, Position { x, y }: Position) -> Vec<Position> {
            let array = &self.backing_array;
            vec![
                y.checked_sub(1).and_then(|y| Some((x, y))), // above
                y.checked_add(1).and_then(|y| Some((x, y))), // below
                x.checked_sub(1).and_then(|x| Some((x, y))), // left
                x.checked_add(1).and_then(|x| Some((x, y))), // right
            ]
            .into_iter()
            .flatten()
            .filter_map(|(x, y)| array.get((x, y)).and(Some(Position { x, y })))
            .collect()
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
