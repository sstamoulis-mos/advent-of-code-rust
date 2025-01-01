advent_of_code::solution!(4);

use ndarray::Array2;

pub fn part_one(input: &str) -> Option<u64> {
    let xmas = Array2::from_shape_vec((1, 4), "XMAS".chars().collect()).ok()?;
    let xmas_rev = xmas.clone().reversed_axes();
    println!("{:#?}", xmas);
    println!("{:#?}", xmas_rev);
    let data = Array2::from_shape_vec(
        (input.lines().count(), input.lines().next()?.chars().count()),
        input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>(),
    )
    .ok()?;
    println!("{:#?}", data);

    let mut count = 0;
    count += data
        .windows(xmas.raw_dim())
        .into_iter()
        .filter(|v| v == &xmas)
        .count() as u64;
    // count += input
    //     .lines()
    //     .map(|line| line.matches(XMAS).count() as u64)
    //     .sum::<u64>();
    // count += input
    //     .lines()
    //     .map(|line| line.chars().rev().collect::<String>().matches(XMAS).count() as u64)
    //     .sum::<u64>();
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
