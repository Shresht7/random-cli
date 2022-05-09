use clap::Subcommand;

mod number;
mod roll;
mod select;

pub use number::*;
pub use roll::*;
pub use select::*;

#[derive(Subcommand)]
pub enum Commands {
    Number(Number),
    Roll(Roll),
    Select(Select),
}
