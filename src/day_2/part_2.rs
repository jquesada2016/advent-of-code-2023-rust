use super::{game, GameReveal};

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(game)
        .map(|res| res.expect("input to parse successfully").1)
        .map(|game| game.reveals)
        .map(|reveals| {
            reveals
                .into_iter()
                .map(|reveal| reveal.accumulate_max(Default::default()))
                .fold(Default::default(), GameReveal::accumulate_max)
        })
        .map(
            |GameReveal {
                 reds,
                 greens,
                 blues,
             }| reds * greens * blues,
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{super::tests::SAMPLE_INPUT, *};

    #[test]
    fn passes_sample_input() {
        let res = solution(SAMPLE_INPUT);

        assert_eq!(res, 2286);
    }
}
