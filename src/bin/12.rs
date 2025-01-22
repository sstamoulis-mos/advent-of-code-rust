advent_of_code::solution!(12);

mod part_one {
    use std::collections::HashSet;

    pub struct Region {
        pub letter: char,
        positions: HashSet<Position>,
    }

    impl Region {
        pub fn area(&self) -> u64 {
            self.positions.len().try_into().unwrap()
        }

        pub fn perimeter(&self) -> u64 {
            let mut remaining_positions: Vec<_> = self.positions.iter().copied().collect();
            let mut perimeter = 0;
            while let Some(position) = remaining_positions.pop() {
                perimeter += position
                    .get_adjacent_positions()
                    .into_iter()
                    .filter(|a| !self.positions.contains(a))
                    .count();
                if position.x == 0 {
                    perimeter += 1;
                }
                if position.y == 0 {
                    perimeter += 1;
                }
            }
            perimeter.try_into().unwrap()
        }

        pub fn sides(&self) -> u64 {
            todo!()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Position {
        pub x: usize,
        pub y: usize,
    }

    impl Position {
        pub fn into_index(self) -> [usize; 2] {
            self.into()
        }

        pub fn get_adjacent_positions(&self) -> Vec<Self> {
            let Position { x, y } = *self;
            vec![
                y.checked_sub(1).map(|y| Position { x, y }), // above
                y.checked_add(1).map(|y| Position { x, y }), // below
                x.checked_sub(1).map(|x| Position { x, y }), // left
                x.checked_add(1).map(|x| Position { x, y }), // right
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    impl From<Position> for [usize; 2] {
        fn from(Position { x, y }: Position) -> Self {
            [y, x]
        }
    }

    impl From<(usize, usize)> for Position {
        fn from((y, x): (usize, usize)) -> Self {
            Position { x, y }
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
            let mut known_positions = HashSet::new();
            while let Some(position) = self
                .backing_array
                .indexed_iter()
                .map(|(index, _)| Position::from(index))
                .find(|p| !known_positions.contains(p))
            {
                if let Some(region) = self.get_region_at(position) {
                    for position in region.positions.iter().copied() {
                        known_positions.insert(position);
                    }
                    regions.push(region);
                }
            }
            regions
        }

        pub fn get_region_at(&self, position: Position) -> Option<Region> {
            let letter = *self.backing_array.get(position.into_index())?;
            let mut positions = HashSet::new();
            let mut checked_positions = HashSet::new();
            let mut possible_positions = vec![position];
            while let Some(possible_position) = possible_positions.pop() {
                if !checked_positions.contains(&possible_position) {
                    checked_positions.insert(possible_position);
                    match self.backing_array.get(possible_position.into_index()) {
                        Some(value) if value.eq(&letter) => {
                            positions.insert(possible_position);
                            possible_positions.extend(
                                self.get_adjacent_positions(possible_position)
                                    .into_iter()
                                    .filter(|p| !checked_positions.contains(p)),
                            );
                        }
                        _ => (),
                    }
                }
            }
            Some(Region { letter, positions })
        }

        fn get_adjacent_positions(&self, position: Position) -> Vec<Position> {
            position
                .get_adjacent_positions()
                .into_iter()
                .filter(|p| self.backing_array.get(p.into_index()).is_some())
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
        // .inspect(|r| {
        //     let letter = r.letter;
        //     let area = r.area();
        //     let perimeter = r.perimeter();
        //     let price = area * perimeter;
        //     println!("{letter}: {area} * {perimeter} = {price}")
        // })
        .map(|r| (r.area(), r.perimeter()))
        .map(|(area, perimeter)| area * perimeter)
        .sum();
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u64> {
    use part_one::Map;

    let map = Map::try_from(input).unwrap();
    let regions = map.get_regions();
    let total_price = regions
        .into_iter()
        .inspect(|r| {
            let letter = r.letter;
            let area = r.area();
            let sides = r.sides();
            let price = area * sides;
            println!("{letter}: {area} * {sides} = {price}")
        })
        .map(|r| (r.area(), r.sides()))
        .map(|(area, sides)| area * sides)
        .sum();
    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
