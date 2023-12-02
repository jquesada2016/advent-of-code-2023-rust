use super::{game, GameReveal};

pub fn solution(input: &str) -> usize {
    let constraints = GameReveal {
        reds: 12,
        greens: 13,
        blues: 14,
    };

    sum_valid_game_ids(input, constraints)
}

fn sum_valid_game_ids(input: &str, constraints: GameReveal) -> usize {
    input
        .lines()
        .map(game)
        .map(|res| res.expect("game line to parse successfully").1)
        .filter(|game| {
            let max_cubes_seen = game
                .reveals
                .iter()
                .copied()
                .fold(Default::default(), GameReveal::accumulate_max);

            max_cubes_seen.within_constraints(constraints)
        })
        .fold(0, |acc, game| acc + game.id as usize)
}

#[cfg(test)]
mod tests {
    use super::{super::tests::SAMPLE_INPUT, *};

    #[test]
    fn passes_sample_input() {
        let constraints = GameReveal {
            reds: 12,
            greens: 13,
            blues: 14,
        };

        let res = sum_valid_game_ids(SAMPLE_INPUT, constraints);

        assert_eq!(res, 8);
    }
}

impl GameReveal {
    /// Checks that no color exceeds the maximum alloted.
    fn within_constraints(self, constraints: Self) -> bool {
        constraints.reds >= self.reds
            && constraints.greens >= self.greens
            && constraints.blues >= self.blues
    }
}
