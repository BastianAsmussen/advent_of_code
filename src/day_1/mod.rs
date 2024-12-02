use crate::Day;

const INPUT: &str = include_str!("input.txt");

fn split_input() -> (Vec<u32>, Vec<u32>) {
    let (left, right): (Vec<_>, Vec<_>) = INPUT
        .split_ascii_whitespace()
        .map(|value| value.parse::<u32>().expect("numeric value"))
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    // Extract the second element of each tuple into separate vectors.
    let left: Vec<u32> = left.into_iter().map(|(_, v)| v).collect();
    let right: Vec<u32> = right.into_iter().map(|(_, v)| v).collect();

    (left, right)
}

pub struct Day1Part1;
impl Day for Day1Part1 {
    type Data = u32;

    fn run() -> Self::Data {
        let (mut left, mut right) = split_input();

        left.sort_unstable();
        right.sort_unstable();

        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }
}

pub struct Day1Part2;
impl Day for Day1Part2 {
    type Data = u32;

    fn run() -> Self::Data {
        let (left, right) = split_input();

        let mut frequency = std::collections::HashMap::new();
        for value in right {
            *frequency.entry(value).or_insert(0) += 1;
        }

        left.into_iter()
            .map(|a| a * frequency.get(&a).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let expected = 2_057_374;
        let actual = Day1Part1::run();

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_part2() {
        let expected = 23_177_084;
        let actual = Day1Part2::run();

        assert_eq!(expected, actual);
    }
}
