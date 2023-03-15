use std::str::FromStr;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    let version = u32::from_str(&args[1])?;
    let count = u32::from_str(&args[2])?;

    let url = format!("https://www.uuidtools.com/api/generate/v{}/count/{}", version, count);
    let uuids: Vec<String> = ureq::get(&url)
        .call()?
        .into_json()?;

    uuids.iter().for_each(|uuid| println!("{}", uuid));
    Ok(())
}


