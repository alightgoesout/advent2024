use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::slice::Iter;
use std::str::FromStr;

use crate::{error, Error, Result, Solution};

mod input;

#[derive(Default)]
pub struct Day9;

impl Solution for Day9 {
    fn part_one(&self) -> Result<String> {
        let mut disk_blocks: DiskBlocks = input::INPUT.parse()?;
        disk_blocks.compact()?;
        Ok(format!(
            "Checksum after compacting block by block: {}",
            disk_blocks.checksum()
        ))
    }

    fn part_two(&self) -> Result<String> {
        let mut disk_map: DiskMap = input::INPUT.parse()?;
        disk_map.compact()?;
        Ok(format!(
            "Checksum after compacting file by file: {}",
            disk_map.checksum()
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Block {
    File(usize),
    Free,
}

impl Block {
    fn is_file(&self) -> bool {
        match self {
            Block::File(_) => true,
            Block::Free => false,
        }
    }

    fn is_free(&self) -> bool {
        match self {
            Block::File(_) => false,
            Block::Free => true,
        }
    }
}

#[derive(Eq, PartialEq)]
struct DiskBlocks(Vec<Block>);

impl DiskBlocks {
    #[cfg(test)]
    fn from_map(s: &str) -> Result<Self> {
        let mut blocks = Vec::new();

        for c in s.chars() {
            let block = match c {
                '.' => Block::Free,
                _ => {
                    Block::File(c.to_digit(10).ok_or_else(|| error!("Invalid size: {c}"))? as usize)
                }
            };
            blocks.push(block);
        }

        Ok(DiskBlocks(blocks))
    }

    fn compact(&mut self) -> Result<()> {
        let mut free_space_index = 0;
        let mut file_index = self.0.len() - 1;

        loop {
            while free_space_index < file_index
                && (self.0[file_index].is_free() || self.0[free_space_index].is_file())
            {
                if self.0[file_index].is_free() {
                    file_index -= 1;
                } else if self.0[free_space_index].is_file() {
                    free_space_index += 1;
                }
            }
            if free_space_index < file_index {
                self.0.swap(file_index, free_space_index);
            } else {
                break;
            }
        }

        Ok(())
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, block)| match block {
                Block::File(id) => *id * i,
                Block::Free => 0,
            })
            .sum()
    }
}

impl FromStr for DiskBlocks {
    type Err = Error;

    fn from_str(s: &str) -> Result<DiskBlocks> {
        let mut blocks = Vec::new();

        for (i, c) in s.chars().enumerate() {
            let size = c.to_digit(10).ok_or_else(|| error!("Invalid size: {c}"))?;
            let block = if i % 2 == 0 {
                Block::File(i / 2)
            } else {
                Block::Free
            };
            (0..size).for_each(|_| blocks.push(block))
        }

        Ok(DiskBlocks(blocks))
    }
}

impl Debug for DiskBlocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for block in &self.0 {
            match block {
                Block::File(id) => write!(f, "{id}")?,
                Block::Free => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Eq, PartialEq)]
enum DiskItem {
    File(File),
    Free(usize),
}

impl DiskItem {
    fn is_free(&self) -> bool {
        match self {
            DiskItem::File(_) => false,
            DiskItem::Free(_) => true,
        }
    }

    fn size(&self) -> usize {
        match self {
            DiskItem::File(File { size, .. }) | DiskItem::Free(size) => *size,
        }
    }
}

#[derive(Eq, PartialEq)]
struct DiskMap(Vec<DiskItem>);

impl DiskMap {
    #[cfg(test)]
    fn from_map(s: &str) -> Result<Self> {
        use itertools::Itertools;

        let mut items = Vec::new();

        for (c, group) in &s.chars().chunk_by(|c| *c) {
            let size = group.count();
            let block = match c {
                '.' => DiskItem::Free(size),
                _ => DiskItem::File(File {
                    id: c.to_digit(10).ok_or_else(|| error!("Invalid size: {c}"))? as usize,
                    size,
                }),
            };
            items.push(block);
        }

        Ok(DiskMap(items))
    }

    fn compact(&mut self) -> Result<()> {
        let mut file_index = self.0.len() - 1;

        loop {
            while file_index > 0 && self.0[file_index].is_free() {
                file_index -= 1;
            }
            if file_index > 0 {
                let file_size = self.0[file_index].size();
                let free_index = (0..file_index).find(|i| {
                    let item = &self.0[*i];
                    item.is_free() && item.size() >= file_size
                });
                if let Some(free_index) = free_index {
                    if let Some(DiskItem::Free(free_size)) = self.0.get_mut(free_index) {
                        *free_size -= file_size;
                        let file =
                            std::mem::replace(&mut self.0[file_index], DiskItem::Free(file_size));
                        self.0.insert(free_index, file);
                    }
                } else {
                    file_index -= 1;
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut block_index = 0;

        for item in self {
            match item {
                DiskItem::File(File { id, size }) => {
                    for i in 0..*size {
                        checksum += *id * (block_index + i);
                    }
                    block_index += size;
                }
                DiskItem::Free(size) => {
                    block_index += size;
                }
            }
        }

        checksum
    }
}

impl FromStr for DiskMap {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = Vec::new();

        for (i, c) in s.chars().enumerate() {
            let size = c.to_digit(10).ok_or_else(|| error!("Invalid size: {c}"))? as usize;
            let item = if i % 2 == 0 {
                DiskItem::File(File { id: i / 2, size })
            } else {
                DiskItem::Free(size)
            };
            items.push(item);
        }

        Ok(DiskMap(items))
    }
}

impl Debug for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for block in self {
            match block {
                DiskItem::File(File { id, size }) => {
                    for _ in 0..*size {
                        write!(f, "{id}")?
                    }
                }
                DiskItem::Free(size) => {
                    for _ in 0..*size {
                        write!(f, ".")?
                    }
                }
            }
        }
        Ok(())
    }
}

impl Index<usize> for DiskMap {
    type Output = DiskItem;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IntoIterator for &'a DiskMap {
    type Item = &'a DiskItem;
    type IntoIter = Iter<'a, DiskItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn parse_and_debug_blocks() {
        let disk_block: DiskBlocks = "123".parse().unwrap();

        assert_eq!(format!("{disk_block:?}"), "0..111",);
    }

    #[test]
    fn parse_and_debug_blocks_example() {
        let disk_block: DiskBlocks = EXAMPLE.parse().unwrap();

        assert_eq!(
            format!("{disk_block:?}"),
            "00...111...2...333.44.5555.6666.777.888899",
        );
    }

    #[test]
    fn test_compact_blocks() {
        let test_data = vec![
            (".1", "1."),
            (".12", "21."),
            ("..12", "21.."),
            ("0..111....22222", "022111222......"),
            (
                "00...111...2...333.44.5555.6666.777.888899",
                "0099811188827773336446555566..............",
            ),
        ];

        for (input, expected) in &test_data {
            let mut disk_blocks = DiskBlocks::from_map(input).unwrap();
            let expected = DiskBlocks::from_map(expected).unwrap();

            disk_blocks.compact().unwrap();

            assert_eq!(disk_blocks, expected)
        }
    }

    #[test]
    fn checksum_example() {
        let disk_block =
            DiskBlocks::from_map("0099811188827773336446555566..............").unwrap();

        assert_eq!(disk_block.checksum(), 1928);
    }

    #[test]
    fn parse_and_debug_disk_map() {
        let disk_map: DiskMap = "123".parse().unwrap();

        assert_eq!(format!("{disk_map:?}"), "0..111",);
    }

    #[test]
    fn parse_and_debug_disk_map_example() {
        let disk_map: DiskMap = EXAMPLE.parse().unwrap();

        assert_eq!(
            format!("{disk_map:?}"),
            "00...111...2...333.44.5555.6666.777.888899",
        );
    }

    #[test]
    fn test_compact_disk_map() {
        let test_data = vec![
            (".1", "1."),
            (".12", "21."),
            ("..12", "21.."),
            ("0..111....22222", "0..111....22222"),
            ("0...111.....22222", "0111...22222....."),
            (
                "00...111...2...333.44.5555.6666.777.888899",
                "00992111777.44.333....5555.6666.....8888..",
            ),
        ];

        for (input, expected) in &test_data {
            let mut disk_map = DiskMap::from_map(input).unwrap();

            disk_map.compact().unwrap();

            assert_eq!(format!("{disk_map:?}"), *expected)
        }
    }
}
