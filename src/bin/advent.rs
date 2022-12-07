use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

// Need to return (move to caller) Vec<String> and not Vec<&str>, because we can't borrow
// something going out of scope. I didn't find the compiler errors helpful here until I
// got closer to figuring that out.
pub fn load_lines(subpath: &str) -> Vec<String> {
    let formatted = format!("./input/{}", subpath);
    let path = Path::new(formatted.as_str());
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<_,_>>().unwrap();

    lines
}
