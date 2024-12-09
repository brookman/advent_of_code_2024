use crate::common::*;
use std::{collections::HashSet, fmt::Display, os::unix::fs::PermissionsExt};

pub struct S;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    Used(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block2 {
    Empty(usize),
    Used(usize, usize),
}

trait Size {
    fn get_size(&self) -> usize;
}

impl Size for Block2 {
    fn get_size(&self) -> usize {
        match self {
            Self::Empty(s) => *s,
            Self::Used(_, s) => *s,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Used(i) => write!(f, "{}", i),
        }
    }
}

impl Solution for S {
    fn solve_one(&self, input: &PuzzleInput) -> String {
        let mut blocks: Vec<Block> = vec![];
        let mut is_block = true;
        for (i, c) in input.lines[0]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
        {
            if is_block {
                for _ in 0..c {
                    blocks.push(Block::Used(i / 2));
                }
            } else {
                for _ in 0..c {
                    blocks.push(Block::Empty);
                }
            }

            is_block = !is_block;
        }

        let mut left = 0;
        let mut right = blocks.len() - 1;
        while left < right {
            while blocks[left] != Block::Empty {
                left += 1;
            }
            while blocks[right] == Block::Empty {
                right -= 1;
            }
            if left < right {
                blocks.swap(left, right);
            }
        }

        let result: usize = blocks
            .into_iter()
            .filter(|b| b != &Block::Empty)
            .enumerate()
            .map(|(i, b)| match b {
                Block::Used(id) => id * i,
                Block::Empty => 0,
            })
            .sum();

        result.to_string()
    }

    fn test_input_one(&self) -> &str {
        r#"2333133121414131402
"#
    }

    fn expected_output_one(&self) -> &str {
        "1928"
    }

    fn solve_two(&self, input: &PuzzleInput) -> String {
        let mut blocks: Vec<Block2> = vec![];
        let mut is_block = true;
        for (i, c) in input.lines[0]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
        {
            if is_block {
                blocks.push(Block2::Used(i / 2, c as usize));
            } else {
                blocks.push(Block2::Empty(c as usize));
            }

            is_block = !is_block;
        }

        let mut right = blocks.len() - 1;

        let mut already_swapped = HashSet::new();

        while right > 0 {
            while matches!(blocks[right], Block2::Empty(_)) {
                right -= 1;
            }

            let mut left = 0;

            while left < right {
                //println!("\n{}", to_string(&blocks, left, right));
                while !matches!(blocks[left], Block2::Empty(_)) {
                    left += 1;
                }

                if left < right {
                    let id = match blocks[right] {
                        Block2::Empty(_) => panic!("must not be empty"),
                        Block2::Used(id, c) => id,
                    };

                    if blocks[left].get_size() >= blocks[right].get_size()
                        && !already_swapped.contains(&id)
                    {
                        let remaining = blocks[left].get_size() - blocks[right].get_size();
                        if remaining > 0 {
                            //println!("remaining {}", remaining);
                            blocks[left] = Block2::Empty(blocks[right].get_size());
                            blocks.insert(left + 1, Block2::Empty(remaining));
                            right += 1;
                        }
                        blocks.swap(left, right);
                        already_swapped.insert(id);

                        break;
                    } else {
                        left += 1;
                    }
                }
            }
            right -= 1;
        }

        //println!("\n{}", to_string(&blocks, 0, right));

        let mut expanded = vec![];
        for block in blocks {
            match block {
                Block2::Empty(i) => {
                    for _ in 0..i {
                        expanded.push(Block::Empty);
                    }
                }
                Block2::Used(i, c) => {
                    for _ in 0..c {
                        expanded.push(Block::Used(i));
                    }
                }
            }
        }
       println!("\n{expanded:?}");

       let result: usize = expanded
       .into_iter()
       .enumerate()
       .map(|(i, b)| match b {
           Block::Used(id) => id * i,
           Block::Empty => 0,
       })
       .sum();

   result.to_string()
    }

    fn test_input_two(&self) -> &str {
        r#"2333133121414131402
"#
    }

    fn expected_output_two(&self) -> &str {
        "2858"
    }
}

fn to_string(blocks: &Vec<Block2>, left: usize, right: usize) -> String {
    let mut result1 = String::new();
    let mut result2 = String::new();

    for (n, block) in blocks.iter().enumerate() {
        match block {
            Block2::Empty(c) => {
                for _ in 0..*c {
                    result1.push('.');
                    if n == left {
                        result2.push('L');
                    } else if n == right {
                        result2.push('R');
                    } else {
                        result2.push(' ');
                    }
                }
            }
            Block2::Used(i, c) => {
                for _ in 0..*c {
                    result1.push_str(&i.to_string());
                    if n == left {
                        result2.push('L');
                    } else if n == right {
                        result2.push('R');
                    } else {
                        result2.push(' ');
                    }
                }
            }
        }
    }

    format!("{}\n{}", result1, result2)
}

fn to_string2(blocks: &Vec<Block>) -> String {
    let mut result = String::new();

    for block in blocks {
        match block {
            Block::Empty => {
                result.push('.');
            }
            Block::Used(i) => {
                result.push_str(&i.to_string());
            }
        }
    }

    result
}
