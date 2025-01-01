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
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();
    let mut instructions_enabled = true;
    Some(
        re.captures_iter(input)
            // .inspect(|c| println!("{:#?}", c))
            .filter_map(|captures| {
                match captures
                    .iter()
                    .skip(1)
                    .filter_map(|c| c)
                    .map(|m| m.as_str())
                    .next()
                {
                    Some("mul") if instructions_enabled => {
                        let a = captures.get(2)?;
                        let b = captures.get(3)?;
                        Some(a.as_str().parse::<u64>().ok()? * b.as_str().parse::<u64>().ok()?)
                    }
                    Some("do") => {
                        instructions_enabled = true;
                        None
                    }
                    Some("don't") => {
                        instructions_enabled = false;
                        None
                    }
                    _ => None,
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
