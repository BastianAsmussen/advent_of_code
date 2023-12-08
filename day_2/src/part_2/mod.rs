use std::collections::HashMap;
use std::hash::BuildHasher;
use crate::part_1::Game;

/// Given a game, return whether or not it is possible.
///
/// # Arguments
///
/// * `game` - A tuple of the game ID and a list of rounds.
///
/// # Returns
///
/// * `HashMap<String, u32>` - A map of colors to counts, where the count is the maximum number of cubes of that color that can be used to play the game.
#[must_use]
pub fn is_game_possible(game: &Game) -> HashMap<String, u32> {
    let mut bag = HashMap::new();

    for round in &game.1 {
        for (color, &count) in round {
            let current_count = bag.entry(color.clone()).or_insert(0);
            if count > *current_count {
                *current_count = count;
            }
        }
    }

    bag
}

/// Given a bag, return the power of the bag.
///
/// # Arguments
///
/// * `bag` - A map of colors to counts, where the count is the maximum number of cubes of that color that can be used to play the game.
///
/// # Generics
///
/// * `S` - The hasher for the bag.
///
/// # Returns
///
/// * `u32` - The power of the bag.
#[must_use]
pub fn calculate_power<S: BuildHasher>(bag: &HashMap<String, u32, S>) -> u32 {
    bag.values().product()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::{Result};
    use crate::part_1::parse_game;

    use super::*;

    #[test]
    fn test_is_game_possible() -> Result<()> {
        let inputs = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ];
        let games = inputs.iter().map(|input| parse_game(input)).collect::<Result<Vec<_>>>()?;
        let bags = games.iter().map(is_game_possible).collect::<Vec<_>>();
        let powers = bags.iter().map(calculate_power).collect::<Vec<_>>();

        assert_eq!(powers, [48, 12, 1560, 630, 36]);

        Ok(())
    }
}
