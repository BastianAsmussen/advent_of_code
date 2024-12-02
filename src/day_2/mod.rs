use crate::Day;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq)]
enum Trend {
    Increase,
    Decrease,
}

impl From<i16> for Trend {
    fn from(diff: i16) -> Self {
        if diff > 0 {
            Self::Decrease
        } else {
            Self::Increase
        }
    }
}

pub struct Day2Part1;
impl Day2Part1 {
    fn is_safe_report(report: &str) -> bool {
        let levels: Vec<i16> = report
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("numeric value"))
            .collect();

        // Short reports are considered safe by default.
        if levels.len() < 2 {
            return true;
        }

        let diffs: Vec<i16> = levels.windows(2).map(|w| w[0] - w[1]).collect();
        let trend = Trend::from(diffs[0]);

        diffs
            .iter()
            .all(|&diff| (1..=3).contains(&diff.abs()) && Trend::from(diff) == trend)
    }
}

impl Day for Day2Part1 {
    type Data = usize;

    fn run() -> Self::Data {
        INPUT
            .lines()
            .filter(|&report| Self::is_safe_report(report))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        let expected = 246;
        let actual = Day2Part1::run();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_safe_report() {
        assert!(Day2Part1::is_safe_report("7 6 4 2 1")); // Safe
        assert!(!Day2Part1::is_safe_report("1 2 7 8 9")); // Unsafe
        assert!(!Day2Part1::is_safe_report("9 7 6 2 1")); // Unsafe
        assert!(!Day2Part1::is_safe_report("1 3 2 4 5")); // Unsafe
        assert!(!Day2Part1::is_safe_report("8 6 4 4 1")); // Unsafe
        assert!(Day2Part1::is_safe_report("1 3 6 7 9")); // Safe
    }
}
