use std::{cell::RefCell, fmt::Display, ops::Deref, rc::Rc};

use ndarray::{s, Array};

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

#[derive(Debug)]
struct Node {
    value: (usize, usize),
    children: Vec<Rc<RefCell<Node>>>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_input(input)?;
    // println!("{}", map);
    let roots: Vec<_> = map
        .trailheads()
        .into_iter()
        .map(|v| {
            Rc::new(RefCell::new(Node {
                value: v,
                children: vec![],
            }))
        })
        .collect();
    let mut unprocessed: Vec<_> = roots.iter().cloned().collect();
    while let Some(node) = unprocessed.pop() {
        let (row, column) = node.borrow().value;
        let v = map[(row, column)];
        for next_value in map
            .indexed_iter()
            .filter_map(|(i, e)| (*e == v + 1).then_some(i))
            .filter(|(r, c)| {
                (row.saturating_sub(1)..=row + 1).contains(r)
                    && (column.saturating_sub(1)..=column + 1).contains(c)
            })
        {
            let next_node = Rc::new(RefCell::new(Node {
                value: next_value,
                children: Vec::new(),
            }));
            node.borrow_mut().children.push(next_node.clone());
            unprocessed.push(next_node);
        }
    }
    // println!("{:?}", roots);
    let count = roots
        .iter()
        .map(|h| {
            let mut count = 0;
            let mut children = h.borrow().children.clone();
            while let Some(node) = children.pop() {
                if map[node.borrow().value] == 9 {
                    count += 1;
                }
                for child in node.borrow().children.iter() {
                    children.push(child.clone());
                }
            }
            count
        })
        .sum();
    Some(count)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
