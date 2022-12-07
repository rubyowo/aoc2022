use crate::days::Day;

use nom::{
    character::complete::{line_ending, u32},
    combinator::map,
    multi::{count, separated_list0},
    IResult,
};

pub struct Day1;

impl Day for Day1 {
    type Input = Vec<Vec<usize>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            count(line_ending, 2),
            separated_list0(line_ending, map(u32, |c| c as usize)),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|e| e.iter().sum()).max().unwrap_or(0)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut max = [0, 0, 0];
        for cals in input.iter().map(|e| e.iter().sum()) {
            if cals > max[0] {
                max[0] = cals;
                max.sort();
            }
        }
        max.iter().sum()
    }
}
