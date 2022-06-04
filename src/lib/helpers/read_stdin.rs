//  Library
use std::io::Read;

/// Read Stdin and append lines to the given vector
pub fn read_stdin_into(vector: &mut Vec<String>) {
    if atty::is(atty::Stream::Stdin) {
        return;
    }
    //  Read input from stdin
    let mut lines = String::new();
    //  Read stdin only if redirected using a pipe
    std::io::stdin().read_to_string(&mut lines).unwrap();
    //  Push to entries
    for line in lines.lines() {
        vector.push(String::from(line));
    }
}
