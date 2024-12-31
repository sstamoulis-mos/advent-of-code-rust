use std::ops::Sub;

advent_of_code::solution!(2);

fn read_to_lists(input: &str) -> Option<Vec<Vec<u64>>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lists = read_to_lists(input)?;
    let results = lists.iter().map(|list| {
        list.windows(2).map(|[a,b]|i64::sub(*b, *a))
    }).collect();
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
