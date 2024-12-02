use crate::Day;

const INPUT: &str = include_str!("input.txt");

pub struct Day1Part1;
impl Day for Day1Part1 {
    type Data = u32;

    fn run() -> Self::Data {
        Self::sanitize(INPUT)
    }

    fn sanitize(input: &str) -> Self::Data {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for (i, value) in input.split_whitespace().enumerate() {
            let value: u32 = value.parse().expect("numeric value");
            if i % 2 == 0 {
                left.push(value);
            } else {
                right.push(value);
            }
        }

        left.sort_unstable();
        right.sort_unstable();

        left.iter()
            .zip(right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum()
    }
}

pub struct Day1Part2;
impl Day for Day1Part2 {
    type Data = u32;

    fn run() -> Self::Data {
        Self::sanitize(INPUT)
    }

    fn sanitize(input: &str) -> Self::Data {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for (i, value) in input.split_whitespace().enumerate() {
            let value: u32 = value.parse().expect("numeric value");
            if i % 2 == 0 {
                left.push(value);
            } else {
                right.push(value);
            }
        }

        left.iter()
            .map(|a| *a * right.iter().filter(|b| a == *b).count() as u32)
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
