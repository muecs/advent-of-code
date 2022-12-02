use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum PartValues {
    A,
    B,
}

#[deny(missing_docs)]
#[derive(Parser, Debug)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    /// AoC event year
    #[clap(value_parser)]
    pub year: u16,

    /// Day of puzzle
    #[clap(value_parser)]
    pub day: u8,

    /// Part of puzzle
    #[clap(value_parser)]
    pub part: PartValues,
}
