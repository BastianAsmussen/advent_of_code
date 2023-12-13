use std::collections::HashMap;
use color_eyre::eyre::Result;
use day_4::Card;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let (cards, duration) = utils::time_it(|| {
        input
            .trim()
            .lines()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()
    });
    let cards = cards?;
    println!("Parsing took {duration:#?}.");

    let (points, duration) = utils::time_it(|| cards.iter().map(Card::points).sum::<usize>());
    println!("Calculating points took {duration:#?}.");

    println!("Part 1: {points} points.");

    let card_map = cards.iter().map(|c| (c.id, c)).collect::<HashMap<_, _>>();
    let (copies, duration) = utils::time_it(|| {
        cards
            .iter()
            .map(|c| c.calculate_won_cards(&card_map).unwrap_or_default())
            .sum::<usize>()
    });
    println!("Calculating copies took {duration:#?}.");

    println!("Part 2: {copies} copies.");

    Ok(())
}
