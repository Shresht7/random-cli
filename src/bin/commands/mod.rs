//  Library
use clap::Subcommand;

//  ========
//  COMMANDS
//  ========

mod number;
mod roll;
mod select;
mod shuffle;
mod strings;
mod toss;
mod uuid;

pub use number::*;
pub use roll::*;
pub use select::*;
pub use shuffle::*;
pub use strings::*;
pub use toss::*;
pub use uuid::*;

#[derive(Subcommand)]
pub enum Commands {
    Number(Number),

    Roll(Roll),

    Select(Select),

    Shuffle(Shuffle),

    String(Strings),

    Toss(Toss),

    #[clap(alias = "guid")]
    Uuid(Uuid),
}
