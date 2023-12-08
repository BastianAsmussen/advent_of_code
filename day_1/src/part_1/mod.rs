/// Calibrate the given input.
///
/// # Arguments
///
/// * `input` - The input to calibrate.
///
/// # Returns
///
/// * `u32` - The calibrated result.
///
/// # Examples
///
/// ```
/// use day_1::part_1::calibrate_lines;
///
/// let input = r"
///     1abc2
///     pqr3stu8vwx
///     a1b2c3d4e5f
///     treb7uchet
/// ".trim();
///
/// assert_eq!(calibrate_lines(input), 142);
/// ```
pub fn calibrate_lines(input: &str) -> u32 {
    input.lines()
        .filter_map(get_digits)
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn get_digits(input: &str) -> Option<(u32, u32)> {
    /*
     We need to get the first and the last digit, ignore the rest.
     These digits might not be at the start and end of the string, so we need to find them.
     There might be no digits or only one digit. If there's only one digit, it's the first and last digit.
     */
    let mut first = None;
    let mut last = None;

    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .for_each(|digit| {
            if first.is_none() {
                first = Some(digit);
            }

            last = Some(digit);
        });

    first.map(|f| (f, last.unwrap_or(f)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_lines() {
        let input = r"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ".trim();

        assert_eq!(calibrate_lines(input), 142);
    }

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits("1abc2"), Some((1, 2)));
        assert_eq!(get_digits("pqr3stu8vwx"), Some((3, 8)));
        assert_eq!(get_digits("a1b2c3d4e5f"), Some((1, 5)));
        assert_eq!(get_digits("treb7uchet"), Some((7, 7)));
    }
}
