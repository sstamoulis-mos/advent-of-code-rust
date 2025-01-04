advent_of_code::solution!(9);

mod common {
    use std::{
        fmt::Display,
        ops::{Deref, DerefMut},
    };

    use ndarray::Array;

    use DiskBlock::{File, Free};

    #[derive(Clone, Copy)]
    pub enum DiskBlock {
        File(u64),
        Free,
    }

    impl Display for DiskBlock {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                File(id) => write!(f, "{}", id),
                Free => write!(f, "."),
            }
        }
    }

    pub struct DiskMap(pub Array<DiskBlock, ndarray::Ix1>);

    impl Deref for DiskMap {
        type Target = Array<DiskBlock, ndarray::Ix1>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for DiskMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Display for DiskMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for e in self.iter() {
                e.fmt(f)?;
            }
            Ok(())
        }
    }

    impl DiskMap {
        pub fn from_input(input: &str) -> Self {
            let mut id = 0;
            Self(Array::from_iter(
                input
                    .chars()
                    .map_while(|c| c.to_digit(10))
                    .enumerate()
                    .flat_map(|(i, e)| {
                        if i % 2 == 0 {
                            // free space
                            let file = (0..e).into_iter().map(|_| File(id)).collect::<Vec<_>>();
                            id += 1;
                            file
                        } else {
                            // file
                            (0..e).into_iter().map(|_| Free).collect::<Vec<_>>()
                        }
                    }),
            ))
        }

        pub fn get_last_file_index(&self, limit: usize) -> Option<usize> {
            (0..limit)
                .rev()
                .filter(|&i| self.get(i).is_some())
                .skip_while(|&i| matches!(self[i], Free))
                .next()
        }
    }
}

mod part_one {
    use std::{fmt::Display, ops::Deref};

    use ndarray::Array;

    use crate::common::DiskBlock::{File, Free};
    use crate::common::DiskMap;

    pub struct CompactDiskMap(Array<u64, ndarray::Ix1>);

    impl Deref for CompactDiskMap {
        type Target = Array<u64, ndarray::Ix1>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Display for CompactDiskMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for e in self.iter() {
                write!(f, "{}", e)?;
            }
            Ok(())
        }
    }

    impl From<DiskMap> for CompactDiskMap {
        fn from(value: DiskMap) -> Self {
            let mut map = value;
            let mut j = map.get_last_file_index(map.len()).unwrap();
            for i in 0..map.len() {
                if i >= j {
                    break;
                }
                if let Free = map[i] {
                    map.swap(i, j);
                    j = match map.get_last_file_index(j) {
                        Some(v) => v,
                        None => break,
                    };
                }
            }
            CompactDiskMap(Array::from_iter(map.iter().filter_map(|e| match e {
                File(v) => Some(*v),
                Free => None,
            })))
        }
    }

    impl CompactDiskMap {
        pub fn checksum(&self) -> u64 {
            self.indexed_iter()
                .map(|(i, &e)| u64::try_from(i).unwrap() * e)
                .sum()
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    use common::DiskMap;
    use part_one::CompactDiskMap;
    let map = DiskMap::from_input(input);
    // println!("{}", map);
    let compact_map = CompactDiskMap::from(map);
    // println!("{}", compact_map);
    Some(compact_map.checksum())
}

mod part_two {
    use std::{
        fmt::Display,
        ops::{Deref, Range},
    };

    use itertools::Itertools;
    use ndarray::{s, Array};

    use crate::common::{
        DiskBlock,
        DiskBlock::{File, Free},
        DiskMap,
    };

    pub struct CompactDiskMap(Array<DiskBlock, ndarray::Ix1>);

    impl Deref for CompactDiskMap {
        type Target = Array<DiskBlock, ndarray::Ix1>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn get_range_with_id<'a>(disk: &Array<DiskBlock, ndarray::Ix1>, id: u64) -> Range<usize> {
        let mut iter = disk.indexed_iter().filter_map(|(i, b)| match b {
            File(file_id) if *file_id == id => Some(i),
            _ => None,
        });
        let start = iter.next().unwrap();
        let end = iter.last().unwrap_or(start);
        start..end + 1
    }

    fn get_first_free_space_range(
        disk: &Array<DiskBlock, ndarray::Ix1>,
        size: usize,
    ) -> Option<Range<usize>> {
        for i in 0..disk.len() {
            if i + size <= disk.len()
                && disk
                    .slice(s![i..i + size])
                    .iter()
                    .all(|b| matches!(b, Free))
            {
                return Some(i..i + size);
            }
        }
        None
    }

    impl From<DiskMap> for CompactDiskMap {
        fn from(value: DiskMap) -> Self {
            let DiskMap(mut disk) = value;
            let max_id = *disk
                .iter()
                .filter_map(|b| match b {
                    File(id) => Some(id),
                    Free => None,
                })
                .max()
                .unwrap();
            for id in (0..=max_id).rev() {
                let file_range = get_range_with_id(&disk, id);
                if let Some(space_range) =
                    get_first_free_space_range(&disk, file_range.clone().count())
                {
                    if space_range.start < file_range.start {
                        file_range
                            .zip_eq(space_range)
                            .for_each(|(a, b)| disk.swap(a, b));
                    }
                }
            }
            Self(disk)
        }
    }

    impl Display for CompactDiskMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for e in self.iter() {
                e.fmt(f)?;
            }
            Ok(())
        }
    }

    impl CompactDiskMap {
        pub fn checksum(&self) -> u64 {
            self.indexed_iter()
                .filter_map(|(i, b)| match b {
                    File(id) => Some(*id * u64::try_from(i).unwrap()),
                    Free => None,
                })
                .sum()
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    use common::DiskMap;
    use part_two::CompactDiskMap;
    let map = DiskMap::from_input(input);
    // println!("{}", map);
    let compact_map = CompactDiskMap::from(map);
    // println!("{}", compact_map);
    Some(compact_map.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
