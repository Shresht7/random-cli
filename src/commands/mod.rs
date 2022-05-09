use clap::Subcommand;

mod number;
mod roll;
mod select;
mod strings;

pub use number::*;
pub use roll::*;
pub use select::*;
pub use strings::*;

#[derive(Subcommand)]
pub enum Commands {
    Number(Number),
    Roll(Roll),
    Select(Select),
    String(Strings),
}
