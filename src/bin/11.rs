use std::{fmt::Display, ops::Rem};

advent_of_code::solution!(11);

struct Stone(u64);

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Stone {
    pub fn blink(self) -> (Self, Option<Self>) {
        if self.0 == 0 {
            return (Self(1), None);
        }
        let number_of_digits = ((self.0 as f64).log10() + 1.) as u32;
        if number_of_digits % 2 == 0 {
            let divider = 10u64.pow(number_of_digits);
            return (Self(self.0 / divider), Some(Self(self.0 % divider)));
        }
        todo!()
    }
}

struct StoneCollection(Vec<Stone>);

impl Display for StoneCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for stone in iter {
            write!(f, " {}", stone)?;
        }
        Ok(())
    }
}

impl StoneCollection {
    fn from_input(input: &str) -> Self {
        Self(
            input
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .map(|i| Stone(i))
                .collect(),
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = StoneCollection::from_input(input);
    println!("{}", stones);
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
