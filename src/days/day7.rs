use crate::days::Day;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, not_line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::{pair, tuple},
    IResult,
};

pub struct Day7;

#[derive(Debug)]
pub enum LogLine {
    Cd(String),
    Dir(String),
    File(u64),
    Ls,
}

fn increase(dir_stack: &mut [u64], size: &u64) {
    let len = dir_stack.len();
    if len > 0 {
        dir_stack[len - 1] += size;
    }
}

impl Day for Day7 {
    type Input = Vec<LogLine>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            alt((
                map(pair(tag("$ cd"), not_line_ending::<&str, _>), |(_, dir)| {
                    LogLine::Cd(dir.to_string())
                }),
                map(pair(tag("dir"), not_line_ending::<&str, _>), |(_, dir)| {
                    LogLine::Dir(dir.to_string())
                }),
                map(
                    tuple((u64, char(' '), not_line_ending::<&str, _>)),
                    |(size, _, _)| LogLine::File(size as u64),
                ),
                map(tag("$ ls"), |_| LogLine::Ls),
            )),
        )(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut sum = 0;
        let mut dir_stack: Vec<u64> = vec![];
        for line in input {
            match line {
                LogLine::Cd(dir) => match dir.replace(" ", "").as_str() {
                    ".." => {
                        let size = dir_stack.pop().unwrap();
                        increase(&mut dir_stack, &size);
                        if size <= 100_000 {
                            sum += size;
                        }
                    }
                    "/" | _ => dir_stack.push(0),
                },
                LogLine::File(size) => increase(&mut dir_stack, &size),
                _ => {}
            }
        }

        while let Some(size) = dir_stack.pop() {
            if size <= 100_000 {
                sum += size;
            }

            increase(&mut dir_stack, &size);
        }

        sum
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut sum = 0;
        let mut dir_stack: Vec<u64> = vec![];
        let mut sizes: Vec<u64> = vec![];
        for line in input {
            match line {
                LogLine::Cd(dir) => match dir.replace(" ", "").as_str() {
                    ".." => {
                        let size = dir_stack.pop().unwrap();
                        sizes.push(size);
                        increase(&mut dir_stack, &size);
                    }
                    "/" | _ => dir_stack.push(0),
                },
                LogLine::File(size) => {
                    sum += size;
                    increase(&mut dir_stack, &size)
                },
                _ => {}
            }
        }

        while let Some(size) = dir_stack.pop() {
            sizes.push(size);
            increase(&mut dir_stack, &size);
        }

        sizes.into_iter().filter(|&size| size >= (sum + 30000000 - 70000000)).min().unwrap()
    }
}
