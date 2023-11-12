//  Library
use is_terminal::IsTerminal;
use std::io::Read;

/// Read Stdin and append lines to the given vector
pub fn read_stdin_into(vector: &mut Vec<String>) {
    // Standard input
    let mut stdin = std::io::stdin();

    //  Return early if stdin is from a tty
    if stdin.is_terminal() {
        return;
    }

    //  Read input from stdin
    let mut lines = String::new();
    //  Read stdin only if redirected using a pipe
    stdin.read_to_string(&mut lines).unwrap();
    //  Push to entries
    for line in lines.lines() {
        vector.push(String::from(line));
    }
}
