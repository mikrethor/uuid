use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    let count = usize::from_str(&args[2])?;

    let content = fs::read_to_string(file_path);

    let uuids: Vec<String> = serde_json::from_str(&content?)?;

    uuids.iter().take(count).for_each(|uuid| println!("{}", uuid));

    Ok(())
}