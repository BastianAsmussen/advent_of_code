use crate::Day;

pub struct Day1;
impl Day for Day1 {
    type Data = u32;

    fn run() -> Self::Data {
        let input = include_str!("./input.txt");

        Self::sanitize(input)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let expected = 2_057_374;
        let actual = Day1::run();

        assert_eq!(expected, actual);
    }
}
