use crate::Day;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq)]
enum Trend {
    Increase,
    Decrease,
}

impl From<bool> for Trend {
    fn from(value: bool) -> Self {
        if value {
            Self::Increase
        } else {
            Self::Decrease
        }
    }
}

pub struct Day2;
impl Day for Day2 {
    type Data = i16;

    fn run() -> Self::Data {
        let mut safe_reports = 0;
        for report in INPUT.lines() {
            let levels = report
                .split_ascii_whitespace()
                .map(|n| n.parse::<i16>().expect("numeric value"));
            let trend = {
                let mut levels = levels.clone().take(2);
                match (levels.next(), levels.next()) {
                    (Some(a), Some(b)) => (a - b > 0).into(),
                    (_, _) => Trend::Increase,
                }
            };

            let mut previous_level = None;
            let mut is_safe = true;

            for level in levels {
                match previous_level {
                    Some(n) => {
                        let diff: i16 = n - level;
                        if trend != (diff > 0).into() || !(1..=3).contains(&diff.abs()) {
                            is_safe = false;

                            break;
                        }

                        previous_level = Some(level);
                    }
                    None => previous_level = Some(level),
                }
            }

            if is_safe {
                safe_reports += 1;
            }
        }

        safe_reports
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day2() {
        let expected = 14;
        let actual = Day2::run();

        assert_eq!(expected, actual);
    }
}
