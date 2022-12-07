use crate::days::Day;

use nom::{
    branch::alt,
    character::complete::{char, line_ending, space1},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

fn parse_move(input: char) -> isize {
    match input {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => unreachable!(),
    }
}

fn parse_outcome(input: char) -> isize {
    match input {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => unreachable!(),
    }
}

fn parse_chars1(input: &[(char, char)]) -> impl Iterator<Item = (isize, isize)> + '_ {
    input.iter().map(|(o, y)| (parse_move(*o), parse_move(*y)))
}

fn parse_chars2(input: &[(char, char)]) -> impl Iterator<Item = (isize, isize)> + '_ {
    input.iter().map(|(o, y)| (parse_move(*o), parse_outcome(*y)))
}

pub struct Day2;

impl Day for Day2 {
    type Input = Vec<(char, char)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                tuple((
                    alt((char('A'), char('B'), char('C'))),
                    space1,
                    alt((char('X'), char('Y'), char('Z'))),
                )),
                |(a, _, x)| (a, x),
            ),
        )(input)
    }

    type Output1 = isize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut score = 0;
        for (opp, you) in parse_chars1(input) {
            score += match you - opp {
                -2 | 1 => 6,
                0 => 3,
                _ => 0,
            } + you;
        }
        score
    }

    type Output2 = isize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut score = 0;
        let win = [2, 3, 1];
        let lose = [3, 1, 2];

        for (opp, out) in parse_chars2(input) {
            score += match out {
                3 => opp,
                6 => win[(opp - 1) as usize],
                0 => lose[(opp - 1) as usize],
                _ => unreachable!()
            } + out;
        }
        score
    }
}
