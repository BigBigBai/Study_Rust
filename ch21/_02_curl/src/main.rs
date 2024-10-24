use reqwest::Error;
use structopt::StructOpt;
use std::path::PathBuf;

// Define a command-line structure using StructOpt
#[derive(Debug, StructOpt)]
#[structopt(name = "cli-example", about = "An example using structopt")]
struct Cli {
    /// The url to process
    url: String,

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Set the output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let args = Cli::from_args();
    // println!("{:#?}", args);
    
    let url = args.url.clone();
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    // let json_body: serde_json::Value = response.json().await?;
    

    println!("Requesting URL: {}", args.url);
    println!("Method: GET");
    println!("Response body:\n{}", body);
    // println!("{:#?}", json_body);    


    Ok(())
}
