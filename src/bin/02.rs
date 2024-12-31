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

fn get_permutations<'a>(
    list: &'a [u64],
) -> impl Iterator<Item = impl Iterator<Item = u64> + use<'a>> {
    (0..list.len()).map(move |n| {
        list.iter()
            .enumerate()
            .filter_map(move |(i, x)| (i != n).then_some(*x))
    })
}

fn is_safe(list: &[u64]) -> bool {
    let diff = list
        .windows(2)
        .map(|w| match w {
            &[a, b] => Some((b as i64).sub(a as i64)),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .unwrap();
    let increasing = diff.iter().all(|&x| x >= 0);
    let decreasing = diff.iter().all(|&x| x <= 0);
    if increasing || decreasing {
        let stepped = diff.into_iter().all(|x| {
            let x = x.abs();
            x >= 1 && x <= 3
        });
        stepped
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lists = read_to_lists(input)?;
    let results = lists.iter().map(|list| is_safe(list)).collect::<Vec<_>>();
    // println!("{:#?}", results);
    Some(results.into_iter().filter(|&x| x).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lists = read_to_lists(input)?;
    let results = lists
        .iter()
        .map(|list| {
            is_safe(list) || get_permutations(list).any(|iter| is_safe(&iter.collect::<Vec<_>>()))
        })
        .collect::<Vec<_>>();
    // println!("{:#?}", results);
    Some(results.into_iter().filter(|&x| x).count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
