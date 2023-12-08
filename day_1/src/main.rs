fn main() {
    let input = include_str!("../input.txt");

    let (result, duration) = utils::time_it(|| day_1::part_1::calibrate_lines(input));
    println!("Part 1: {result} (took {duration:#?})");

    let (result, duration) = utils::time_it(|| day_1::part_2::calibrate_lines(input));
    println!("Part 2: {result} (took {duration:#?})");
}
