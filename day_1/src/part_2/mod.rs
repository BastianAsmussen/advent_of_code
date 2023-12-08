static STRINGIFIED_NUMBERS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

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
/// use day_1::part_2::calibrate_lines;
///
/// let input = r"
///    two1nine
///    eightwothree
///    abcone2threexyz
///    xtwone3four
///    4nineeightseven2
///    zoneight234
///    7pqrstsixteen
/// ".trim();
///
/// assert_eq!(calibrate_lines(input), 281);
/// ```
pub fn calibrate_lines(input: &str) -> u32 {
    input.lines()
        .filter_map(get_digits)
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn get_digits(input: &str) -> Option<(u32, u32)> {
    let mut first = None;
    let mut last = None;

    let mut seen_chars = 0;
    for c in input.chars() {
        // Keep a log of how many characters we've seen so far, so we can skip them later.
        seen_chars += 1;

        if let Some(digit) = c.to_digit(10) {
            if first.is_none() {
                first = Some(digit);
            }

            last = Some(digit);

            continue;
        }

        /*
         If the character is not a digit, try parsing it as a stringified number.
         The stringified number might be more than one character long, so we need to check the next characters as well.
         */
        let mut number = String::new();
        for c in input.chars().skip(seen_chars - 1) {
            number.push(c);

            // If the stringified number is successfully parsed, we can stop checking the next characters and move on.
            if let Some(number) = get_number_from_string(&number) {
                if first.is_none() {
                    first = Some(number);
                }

                last = Some(number);

                break;
            }
        }
    }

    first.map(|f| (f, last.unwrap_or(f)))
}

fn get_number_from_string(input: &str) -> Option<u32> {
    for (stringified_number, number) in &STRINGIFIED_NUMBERS {
        if input == *stringified_number {
            return Some(*number);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_lines() {
        let input = r"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ".trim();

        assert_eq!(calibrate_lines(input), 281);
    }

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits("two1nine"), Some((2, 9)));
        assert_eq!(get_digits("eightwothree"), Some((8, 3)));
        assert_eq!(get_digits("abcone2threexyz"), Some((1, 3)));
        assert_eq!(get_digits("xtwone3four"), Some((2, 4)));
        assert_eq!(get_digits("4nineeightseven2"), Some((4, 2)));
        assert_eq!(get_digits("zoneight234"), Some((1, 4)));
        assert_eq!(get_digits("7pqrstsixteen"), Some((7, 6)));
    }
}
