use crate::days::Day;

use itertools::Itertools;
use nom::{
    character::complete::{char, line_ending, u8},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

pub struct Day4;

impl Day for Day4 {
    type Input = Vec<(u8, u8)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                tuple((
                    tuple((u8, char('-'), u8)),
                    char(','),
                    tuple((u8, char('-'), u8)),
                )),
                |(c1, _, c2)| (c1, c2),
            ),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().fold(0, |x, y| x + y as usize)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        0
    }
}
