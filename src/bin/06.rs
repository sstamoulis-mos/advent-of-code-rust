use ndarray::{s, Array, Dimension};

advent_of_code::solution!(6);

enum Heading {
    Up,
    Right,
    Down,
    Left,
}

struct Map {
    array: Array<char, ndarray::Ix2>,
}

impl Map {
    fn from_input(input: &str) -> Option<Self> {
        let shape = (input.lines().count(), input.lines().next()?.chars().count());
        let array =
            Array::from_shape_vec(shape, input.lines().flat_map(|l| l.chars()).collect()).ok()?;
        Some(Self { array })
    }

    fn get_path(&self) -> Option<Vec<(usize,usize)>> {
        let path = Vec::new();
        let map = &self.array;
        let (rows, columns) = map.raw_dim().into_pattern();
        let (mut pos, mut head) = map.indexed_iter().find_map(|(pos, &v)| match v {
            '^' => Some((pos, Heading::Up)),
            '>' => Some((pos, Heading::Right)),
            'v' => Some((pos, Heading::Down)),
            '<' => Some((pos, Heading::Left)),
            _ => None,
        })?;
        path.push(pos);
        'outer: loop {
            let next_pos = {
                let (mut r, mut c) = pos;
                loop {
                    match head {
                        Heading::Up if r > 0 => r -= 1,
                        Heading::Right if c < columns - 1 => c += 1,
                        Heading::Down if r < rows - 1 => r += 1,
                        Heading::Left if c > 0 => c -= 1,
                        _ => break 'outer,
                    }
                    if let Some(&v) = map.get((r, c)) {
                        if v == '#' {
                            break;
                        }
                    }
                }
                (r, c)
            };
            let mut slice = match head {
                Heading::Up => map.slice_mut(s![next_pos.0 + 1..=pos.0, pos.1]),
                Heading::Right => map.slice_mut(s![pos.0, pos.1..next_pos.1]),
                Heading::Down => map.slice_mut(s![pos.0..next_pos.0, pos.1]),
                Heading::Left => map.slice_mut(s![pos.0, next_pos.1 + 1..=pos.1]),
            };
            slice.fill('X');
            pos = match head {
                Heading::Up => (next_pos.0 + 1, next_pos.1),
                Heading::Right => (next_pos.0, next_pos.1 - 1),
                Heading::Down => (next_pos.0 - 1, next_pos.1),
                Heading::Left => (next_pos.0, next_pos.1 + 1),
            };
            head = match head {
                Heading::Up => Heading::Right,
                Heading::Right => Heading::Down,
                Heading::Down => Heading::Left,
                Heading::Left => Heading::Up,
            };
        }
        let mut slice = match head {
            Heading::Up => map.slice_mut(s![..=pos.0, pos.1]),
            Heading::Right => map.slice_mut(s![pos.0, pos.1..]),
            Heading::Down => map.slice_mut(s![pos.0.., pos.1]),
            Heading::Left => map.slice_mut(s![pos.0, ..=pos.1]),
        };
        slice.fill('X');
        Some(path)
    }

    fn with_path(&self) -> Option<Self> {
        let mut map = self.array.clone();
        let (rows, columns) = map.raw_dim().into_pattern();
        let (mut pos, mut head) = map.indexed_iter().find_map(|(pos, &v)| match v {
            '^' => Some((pos, Heading::Up)),
            '>' => Some((pos, Heading::Right)),
            'v' => Some((pos, Heading::Down)),
            '<' => Some((pos, Heading::Left)),
            _ => None,
        })?;
        'outer: loop {
            let next_pos = {
                let (mut r, mut c) = pos;
                loop {
                    match head {
                        Heading::Up if r > 0 => r -= 1,
                        Heading::Right if c < columns - 1 => c += 1,
                        Heading::Down if r < rows - 1 => r += 1,
                        Heading::Left if c > 0 => c -= 1,
                        _ => break 'outer,
                    }
                    if let Some(&v) = map.get((r, c)) {
                        if v == '#' {
                            break;
                        }
                    }
                }
                (r, c)
            };
            let mut slice = match head {
                Heading::Up => map.slice_mut(s![next_pos.0 + 1..=pos.0, pos.1]),
                Heading::Right => map.slice_mut(s![pos.0, pos.1..next_pos.1]),
                Heading::Down => map.slice_mut(s![pos.0..next_pos.0, pos.1]),
                Heading::Left => map.slice_mut(s![pos.0, next_pos.1 + 1..=pos.1]),
            };
            slice.fill('X');
            pos = match head {
                Heading::Up => (next_pos.0 + 1, next_pos.1),
                Heading::Right => (next_pos.0, next_pos.1 - 1),
                Heading::Down => (next_pos.0 - 1, next_pos.1),
                Heading::Left => (next_pos.0, next_pos.1 + 1),
            };
            head = match head {
                Heading::Up => Heading::Right,
                Heading::Right => Heading::Down,
                Heading::Down => Heading::Left,
                Heading::Left => Heading::Up,
            };
        }
        let mut slice = match head {
            Heading::Up => map.slice_mut(s![..=pos.0, pos.1]),
            Heading::Right => map.slice_mut(s![pos.0, pos.1..]),
            Heading::Down => map.slice_mut(s![pos.0.., pos.1]),
            Heading::Left => map.slice_mut(s![pos.0, ..=pos.1]),
        };
        slice.fill('X');
        Some(Self { array: map })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_input(input)?;
    // println!("{:?}", map.array);
    let path = map.with_path()?;
    println!("{:?}", path.array);
    let count = path.array.iter().filter(|&&v| v == 'X').count();
    Some(count as u64)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
