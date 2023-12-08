use color_eyre::eyre::{Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    let (games, duration) = utils::time_it(|| {
        input.lines().map(day_2::part_1::parse_game).collect::<Result<Vec<_>>>()
    });
    let games = games?;
    println!("Part 1: Parsed {length:#?} games in {duration:#?}.", length = games.len());

    let (id_sum, duration) = utils::time_it(|| day_2::part_1::are_games_possible(&games));
    println!("Part 1: Found ID sum ({id_sum}) in {duration:#?}.");

    let (bags, duration) = utils::time_it(|| games.iter().map(day_2::part_2::is_game_possible).collect::<Vec<_>>());
    println!("Part 2: Found {length:#?} bags in {duration:#?}.", length = bags.len());

    let (power, duration) = utils::time_it(|| bags.iter().map(day_2::part_2::calculate_power).sum::<u32>());
    println!("Part 2: Found power ({power}) in {duration:#?}.");

    Ok(())
}
