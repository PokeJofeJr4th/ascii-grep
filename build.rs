use std::{error::Error, fs};

#[path = "src/lib.rs"]
mod lib;

use lib::{CharEntry, CharFile};

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("index.txt")?;
    let entries = content
        .split('\n')
        .flat_map(|row| {
            let mut row_iter = row.split(';');
            let code = row_iter.next()?;
            let description = row_iter.next()?;
            let char_code = u32::from_str_radix(code, 16).ok()?;
            let char = char::from_u32(char_code)?;
            Some(CharEntry {
                char,
                description: description.to_lowercase(),
            })
        })
        .collect::<Vec<_>>();
    let entries = CharFile { rows: entries };
    let bytes = bincode::serialize(&entries)?;
    fs::write("index", bytes)?;
    Ok(())
}
