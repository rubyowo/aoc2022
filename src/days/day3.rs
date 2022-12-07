use crate::days::Day;

use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

pub struct Day3;

impl Day for Day3 {
    type Input = Vec<Vec<u8>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(alpha1, |s: &str| {
                s.chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            return (c as u8) - 38;
                        }
                        (c as u8) - 96
                    })
                    .collect_vec()
            }),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|sack| {
                (
                    sack.split_at(sack.len() / 2)
                )
            })
            .flat_map(|sack| sack.0.into_iter().filter(move |e| sack.1.contains(e)).dedup())
            .map(|e| *e as usize)
            .sum::<usize>()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut sum = 0;
        for sack in input.chunks_exact(3) {
            for item in &sack[0] {
                if sack[1].contains(item) && sack[2].contains(item) {
                    sum += *item as usize;
                    break;
                }
            }
        }
        sum
    }
}
