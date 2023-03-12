use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// version of the uuid to generate 1,2,4
    version: u32,
    /// numbers of uuid to request
    count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = reqwest::Client::new();
    let url = format!("https://www.uuidtools.com/api/generate/v{}/count/{}", args.version, args.count);
    let uuids = client.get(url)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Vec<String>>()
        .await?;

    uuids.iter().for_each(|uuid| println!("{}", uuid));

    Ok(())
}


