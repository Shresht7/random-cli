// Library
use clap::Args;

// ====
// UUID
// ====

/// Generate a random UUID
///
/// Examples:
/// random uuid     - Generates a random uuid
/// random guid     - Generates a random guid (aka uuid)
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Uuid {}

impl Uuid {
    pub fn execute(&self) {
        let id = uuid::Uuid::new_v4();
        println!("{}", id);
    }
}
