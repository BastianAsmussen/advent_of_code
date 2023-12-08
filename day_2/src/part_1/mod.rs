use std::collections::HashMap;
use color_eyre::eyre::{Result};

/// A game, where each game is a tuple of the game ID and a list of rounds.
pub type Game = (u32, Vec<HashMap<String, u32>>);

/// Given a game, return the game ID and a list of rounds.
///
/// # Arguments
///
/// * `game` - A string representing a game.
///
/// # Returns
///
/// * `Result<Game>` - A tuple of the game ID and a list of rounds, where each round is a map of colors to counts.
///
/// # Errors
///
/// * `ParseIntError` - If the game ID cannot be parsed as a `u32`.
pub fn parse_game(game: &str) -> Result<Game> {
    let parts = game.split(": ").collect::<Vec<&str>>();
    let id = parts[0][5..].parse()?;

    let rounds = parts[1].split("; ").map(|round| {
        let mut round_map = HashMap::new();

        for cube in round.split(", ") {
            let cube_parts = cube.split(' ').collect::<Vec<&str>>();
            let count = cube_parts[0].parse()?;
            let color = cube_parts[1].to_string();

            round_map.insert(color, count);
        }

        Ok(round_map)
    }).collect::<Result<Vec<_>>>()?;

    Ok((id, rounds))
}

/// Given a game, return whether or not it is possible.
///
/// # Arguments
///
/// * `game` - A tuple of the game ID and a list of rounds.
///
/// # Returns
///
/// * `bool` - Whether or not the game is possible.
#[must_use]
pub fn is_game_possible(game: &Game) -> bool {
    let bag = [("red", 12), ("green", 13), ("blue", 14)].iter().copied().collect::<HashMap<_, _>>();

    for round in &game.1 {
        for (color, &count) in round {
            if count > *bag.get(color.as_str()).unwrap_or(&0) {
                return false;
            }
        }
    }

    true
}

/// Given a list of games, return the sum of the IDs of the games that are possible.
///
/// # Arguments
///
/// * `games` - A list of games, where each game is a tuple of the game ID and a list of rounds.
///
/// # Returns
///
/// * The sum of the IDs of the games that are possible.
#[must_use]
pub fn are_games_possible(games: &[Game]) -> u32 {
    // Sum up the IDs of the games that are possible.
    games
        .iter()
        .filter(|game| is_game_possible(game))
        .map(|game| game.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (id, rounds) = parse_game(input)?;

        assert_eq!(id, 1);
        assert_eq!(rounds.len(), 3);
        assert_eq!(rounds[0].len(), 2);
        assert_eq!(rounds[1].len(), 3);
        assert_eq!(rounds[2].len(), 1);

        Ok(())
    }

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

        let expected = vec![true, true, false, false, true];
        let actual = games.iter().map(is_game_possible).collect::<Vec<_>>();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_are_games_possible() -> Result<()> {
        let inputs = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ];
        let games = inputs.iter().map(|input| parse_game(input)).collect::<Result<Vec<_>>>()?;

        let expected = 8; // Sum of the IDs of the games that are possible.
        let actual = are_games_possible(&games);

        assert_eq!(expected, actual);

        Ok(())
    }
}
