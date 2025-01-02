advent_of_code::solution!(4);

use ndarray::{s, Array};

pub fn part_one(input: &str) -> Option<u64> {
    let xmas = Array::from_iter("XMAS".chars());
    // println!("{:#?}", xmas);
    let rows = input.lines().count();
    let columns = input.lines().next()?.chars().count();
    let data = Array::from_shape_vec(
        (rows, columns),
        input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>(),
    )
    .ok()?;
    // println!("{:#?}", data);

    let mut count = 0;
    for ((r, c), &v) in data.indexed_iter() {
        if v == 'X' {
            // right
            if c + 4 <= columns {
                let slice = data.slice(s![r, c..c + 4]);
                // println!("R  {:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // right-down
            if c + 4 <= columns && r + 4 <= rows {
                let slice = data.slice(s![r..r + 4, c..c + 4]).into_diag();
                // println!("RD {:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // down
            if r + 4 <= rows {
                let slice = data.slice(s![r..r + 4, c]);
                // println!("D  {:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // down-left
            if r + 4 <= rows && c.checked_sub(3).is_some() {
                let slice = data.slice(s![r..r + 4, c - 3..=c;-1]).into_diag();
                // println!("{:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // left
            if c.checked_sub(3).is_some() {
                let slice = data.slice(s![r, c - 3..=c;-1]);
                // println!("{:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // left-up
            if c.checked_sub(3).is_some() && r.checked_sub(3).is_some() {
                let slice = data.slice(s![r-3..=r;-1, c - 3..=c;-1]).into_diag();
                // println!("{:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // up
            if r.checked_sub(3).is_some() {
                let slice = data.slice(s![r-3..=r;-1, c]);
                // println!("{:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
            // up-right
            if r.checked_sub(3).is_some() && c + 4 <= columns {
                let slice = data.slice(s![r-3..=r;-1, c..c+4]).into_diag();
                // println!("{:#?}", slice);
                if slice == xmas {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mas = Array::from_vec(vec!['M', 'A', 'S']);
    let rows = input.lines().count();
    let columns = input.lines().next()?.chars().count();
    let data = Array::from_shape_vec(
        (rows, columns),
        input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>(),
    )
    .ok()?;
    let mut count = 0;
    for window in data.windows((mas.raw_dim()[0], mas.raw_dim()[0])) {
        if (window.diag() == mas || window.diag().slice(s![..;-1]) == mas)
            && (window.slice(s![..,..;-1]).diag() == mas
                || window.slice(s![..,..;-1]).diag().slice(s![..;-1]) == mas)
        {
            count += 1
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
