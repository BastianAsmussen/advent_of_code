pub mod day_1;

pub trait Day {
    type Data;

    fn run() -> Self::Data;
    fn sanitize(input: &str) -> Self::Data;
}
