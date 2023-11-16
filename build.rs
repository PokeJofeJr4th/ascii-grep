use std::{error::Error, fs};

#[path = "src/lib.rs"]
mod lib;

use lib::{CharEntry, CharFile};

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("index.txt")?;
    let entries = content
        .split('\n')
        .flat_map(|row| {
            println!("{row:?}");
            let mut idx = row.len();
            while idx != 0
                && row
                    .chars()
                    .nth(idx - 1)
                    .as_ref()
                    .is_some_and(char::is_ascii_hexdigit)
            {
                idx -= 1;
            }
            let (lhs, rhs) = row.split_at(idx);
            let char_code = u32::from_str_radix(rhs, 16).ok()?;
            let char = char::from_u32(char_code)?;
            Some(CharEntry {
                char,
                description: lhs.to_lowercase(),
            })
        })
        .collect::<Vec<_>>();
    let entries = CharFile { rows: entries };
    let bytes = bincode::serialize(&entries)?;
    fs::write("index", bytes)?;
    Ok(())
}
