use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, u8},
    combinator::{map, map_parser, value},
    multi::many0,
    IResult,
};

pub fn solution(input: &[u8]) -> usize {
    let input = String::from_utf8_lossy(input);

    input.lines().fold(0, |acc, curr| {
        let (_, numbers) = extract_numbers(curr.as_bytes()).unwrap_or_default();

        let Some(first) = numbers.first().copied() else {
            // If there are no numbers, we're done
            return acc;
        };
        let last = numbers
            .last()
            .copied()
            // If there are no more numbers in the iterator,
            // we act as if the first number is the last number
            .unwrap_or(first);

        let first = first as usize;
        let last = last as usize;

        acc + first * 10 + last
    })
}

fn extract_numbers(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (rest, numbers) = many0(alt((map(number, Some), map(anychar, |_| None))))(input)?;

    let numbers = numbers.into_iter().flatten().collect();

    Ok((rest, numbers))
}

/// Recognizes both digits and worded numbers
///
/// Outputs the value as an ascii digit.
fn number(input: &[u8]) -> IResult<&[u8], u8> {
    alt((map_parser(take(1usize), u8), word_number))(input)
}

/// Recognizes an english-worded number, returning it's value
/// as a `u8` and returning the remaining bytes before
/// the last recognized byte.
///
/// # Example
/// If the input `b"eightwo"` is provided, it will return `two`
/// as the remaining slice.
fn word_number(input: &[u8]) -> IResult<&[u8], u8> {
    let (rest, number) = alt((
        value(0, tag(b"zero")),
        value(1, tag(b"one")),
        value(2, tag(b"two")),
        value(3, tag(b"three")),
        value(4, tag(b"four")),
        value(5, tag(b"five")),
        value(6, tag(b"six")),
        value(7, tag(b"seven")),
        value(8, tag(b"eight")),
        value(9, tag("nine")),
    ))(input)?;

    // Subtract 2 to make the rest slice begin from the last
    // parsed character
    let start = input.len() - rest.len() - 1;

    Ok((&input[start..], number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_sample_input() {
        macro_rules! validate_test_input {
            { $($input:literal => $values:expr),* $(,)? } => {
              $(
                let (_, numbers) = extract_numbers($input).unwrap();

                assert_eq!(numbers, $values);
              )*
            };
        }

        validate_test_input! {
            b"two1nine" => [2, 1, 9],
            b"eightwothree" => [8, 2, 3],
            b"abcone2threexyz" => [1, 2, 3],
            b"xtwone3four" => [2, 1, 3, 4],
            b"4nineeightseven2" => [4, 9, 8, 7, 2],
            b"zoneight234" => [1, 8, 2, 3, 4],
            b"7pqrstsixteen" => [7, 6],
        };
    }

    #[test]
    fn can_read_worded_number() {
        let (rest, n) = number(b"twone").unwrap();

        assert_eq!(n, 2);
        assert_eq!(String::from_utf8_lossy(rest), "one");
    }

    #[test]
    fn can_recognize_number() {
        let (_, n) = number(b"1").unwrap();

        assert_eq!(n, 1);
    }
}
