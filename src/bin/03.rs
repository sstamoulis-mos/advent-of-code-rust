advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|captures| {
            let a = captures.get(1)?;
            let b = captures.get(2)?;
            Some(a.as_str().parse::<u64>().ok()? * b.as_str().parse::<u64>().ok()?)
        })
        // .inspect(|a| println!("{:?}", a))
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
