pub fn solution(input: &str) -> usize {
    // We first split by lines
    input.lines().fold(0, |acc, curr| {
        let mut numbers = curr
            .chars()
            // We remove all letters, since we don't care about them
            .filter(|c| c.is_ascii_digit())
            // convert ascii digit to u8
            .map(|c| c as u8 - b'0');

        // Get the first and last numbers we care about

        let Some(first) = numbers.next() else {
            // If there are no numbers, we're done
            return acc;
        };
        let last = numbers
            .last()
            // If there are no more numbers in the iterator,
            // we act as if the first number is the last number
            .unwrap_or(first);

        let first = first as usize;
        let last = last as usize;

        acc + first * 10 + last
    })
}
