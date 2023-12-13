use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let input = r"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "
    .trim()
    .replace([' ', '\n'], "");

    println!("Part 1: {}", sum_of_parts(&input)?);

    Ok(())
}

fn sum_of_parts(input: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_parts() -> Result<()> {
        let input = r"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "
        .trim()
        .replace([' ', '\n'], "");

        assert_eq!(sum_of_parts(&input)?, 4_361);

        Ok(())
    }
}
