pub mod day_1;
pub mod day_2;

pub trait Day {
    type Data;

    fn run() -> Self::Data;
}
