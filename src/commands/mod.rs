use clap::Subcommand;

//  ========
//  COMMANDS
//  ========

mod number;
mod roll;
mod select;
mod strings;
mod toss;

pub use number::*;
pub use roll::*;
pub use select::*;
pub use strings::*;
pub use toss::*;

#[derive(Subcommand)]
pub enum Commands {
    Number(Number),
    Roll(Roll),
    Select(Select),
    String(Strings),
    Toss(Toss),
}
