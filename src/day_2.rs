pub mod part_1;
pub mod part_2;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space1, u16},
    combinator::{map, opt},
    multi::many1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    id: u16,
    reveals: Vec<GameReveal>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct GameReveal {
    reds: usize,
    greens: usize,
    blues: usize,
}

impl std::ops::Add for GameReveal {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.reds += rhs.reds;
        self.greens += rhs.greens;
        self.blues += rhs.blues;

        self
    }
}

impl std::iter::Sum for GameReveal {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), std::ops::Add::add)
    }
}

impl GameReveal {
    /// Keeps the maximum cubes available for each color.
    fn accumulate_max(self, acc: Self) -> Self {
        let Self {
            reds,
            greens,
            blues,
        } = self;

        let Self {
            reds: acc_reds,
            greens: acc_greens,
            blues: acc_blues,
        } = acc;

        Self {
            reds: reds.max(acc_reds),
            greens: greens.max(acc_greens),
            blues: blues.max(acc_blues),
        }
    }
}

enum Cube {
    Read(usize),
    Green(usize),
    Blue(usize),
}

impl From<Cube> for GameReveal {
    fn from(cube: Cube) -> Self {
        let mut reveal = GameReveal::default();

        match cube {
            Cube::Read(count) => reveal.reds = count,
            Cube::Green(count) => reveal.greens = count,
            Cube::Blue(count) => reveal.blues = count,
        }

        reveal
    }
}

/// Recognizes a single full game input line.
fn game(input: &str) -> IResult<&str, Game> {
    let (rest, (_, _, id, _, reveals)) = tuple((
        tag("Game"),
        space1,
        u16,
        char(':'),
        many1(tuple((space1, reveal, opt(char(';'))))),
    ))(input)?;

    let reveals = reveals.into_iter().map(|(_, reveal, _)| reveal).collect();

    Ok((rest, Game { id, reveals }))
}

/// Recognizes the revealed colors.
fn reveal(input: &str) -> IResult<&str, GameReveal> {
    let (rest, (cube_1, _, cube_2, _, cube_3)) = tuple((
        map(cube_color, GameReveal::from),
        opt(tuple((char(','), space1))),
        opt(map(cube_color, GameReveal::from)),
        opt(tuple((char(','), space1))),
        opt(map(cube_color, GameReveal::from)),
    ))(input)?;

    let cube_2 = cube_2.unwrap_or_default();
    let cube_3 = cube_3.unwrap_or_default();

    Ok((rest, cube_1 + cube_2 + cube_3))
}

/// Recognizes a single cube revelation, like `4 red`, or `1 green`.
fn cube_color(input: &str) -> IResult<&str, Cube> {
    let (rest, (count, color)) =
        separated_pair(u16, space1, alt((tag("red"), tag("green"), tag("blue"))))(input)?;

    let count = count as usize;

    let cube = match color {
        "red" => Cube::Read(count),
        "green" => Cube::Green(count),
        "blue" => Cube::Blue(count),
        color => {
            panic!(
                "invalid game input: expected one of red, green, or \
                 blue, but found `{color}`",
            )
        }
    };

    Ok((rest, cube))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn can_parse_game() {
        SAMPLE_INPUT
            .lines()
            .map(|input| (input, game(input).unwrap()))
            .for_each(|(input, (rest, _))| {
                assert!(rest.is_empty(), "input: {input}\nrest: {}", rest)
            })
    }
}
