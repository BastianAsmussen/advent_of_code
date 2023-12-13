use std::collections::HashMap;
use color_eyre::eyre::{eyre, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// A regex that matches a card.
    static ref CARD_REGEX: Regex = Regex::new(r"Card\s+(\d+):").expect("Failed to compile regex!");
}

/// A card has winning numbers and the actual numbers present on the card.
///
/// # Fields
///
/// * `id` - The ID of the card. (Only available in part 2.)
///
/// * `winning_numbers` - The numbers that are required to win the game.
/// * `actual_numbers` - The numbers that are present on the card.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg(feature = "part_1")]
pub struct Card {
    #[cfg(feature = "part_2")]
    pub id: usize,

    winning_numbers: Vec<u8>,
    actual_numbers: Vec<u8>,
}

impl Card {
    /// Get the points for a card.
    ///
    /// # Returns
    ///
    /// * `usize` - The points for the card.
    #[must_use]
    pub fn points(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|&n| self.actual_numbers.contains(n))
            .fold(None, |points, _| {
                points.map_or(Some(1), |points| Some(points * 2))
            })
            .unwrap_or_default()
    }
    /// Get the number of cards that we've won.
    ///
    /// # Arguments
    ///
    /// * `card_map` - A map containing all the cards.
    ///
    /// # Returns
    ///
    /// * `Option<size>` - The cards that we've won, if any.
    #[cfg(feature = "part_2")]
    #[must_use]
    pub fn calculate_won_cards(&self, card_map: &HashMap<usize, &Self>) -> Option<usize> {
        let mut found_cards = 1;
        let wins = self
            .winning_numbers
            .iter()
            .filter(|&n| self.actual_numbers.contains(n))
            .count();
        if wins == 0 {
            return Some(found_cards);
        }

        for i in 0..wins {
            let id = self.id + i + 1;
            let card = card_map.get(&id)?;

            found_cards += card.calculate_won_cards(card_map)?;
        }

        Some(found_cards)
    }
}

impl TryFrom<&str> for Card {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        /*
        A string should look like this: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        We need to split the string into two parts: the winning numbers and the actual numbers.
        The winning numbers are the numbers before the pipe character, and the actual numbers
        are the numbers after the pipe character.
        */

        // Extract the "Card X: " (ID) part of the string.
        let id = CARD_REGEX
            .captures(value)
            .ok_or_else(|| eyre!("No card ID found!"))?
            .get(1)
            .ok_or_else(|| eyre!("No card ID found!"))?
            .as_str()
            .parse()?;

        // Remove the "Card X: " (ID) part of the string.
        let value = CARD_REGEX.replace(value, "");

        // Split the string into two parts, using the pipe character as the separator.
        let mut parts = value.split('|');

        // The winning numbers are the first part of the string.
        let winning_numbers = parts
            .next()
            .ok_or_else(|| eyre!("No winning numbers found!"))?
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        // The actual numbers are the second part of the string.
        let actual_numbers = parts
            .next()
            .ok_or_else(|| eyre!("No actual numbers found!"))?
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            id,

            winning_numbers,
            actual_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::try_from(input)?;

        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.actual_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        Ok(())
    }

    #[test]
    fn test_get_points() -> Result<()> {
        let input = r"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        let cards = input
            .lines()
            .map(Card::try_from)
            .collect::<Result<Vec<_>>>()?;

        let actual_points = cards.iter().map(Card::points).collect::<Vec<_>>();
        let expected_points = vec![8, 2, 2, 1, 0, 0];

        assert_eq!(actual_points, expected_points);

        Ok(())
    }

    #[cfg(feature = "part_2")]
    #[test]
    fn test_calculate_won_cards() -> Result<()> {
        let input = r"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        let cards = input
            .lines()
            .map(Card::try_from)
            .collect::<Result<Vec<_>>>()?;
        let card_map = cards.iter().map(|c| (c.id, c)).collect::<HashMap<_, _>>();

        let actual_won_cards = cards
            .iter()
            .map(|c| c.calculate_won_cards(&card_map).unwrap_or_default())
            .sum::<usize>();
        let expected_won_cards = 30;

        assert_eq!(actual_won_cards, expected_won_cards);

        Ok(())
    }
}
