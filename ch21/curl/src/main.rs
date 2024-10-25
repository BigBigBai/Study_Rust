use reqwest::Error;
use structopt::StructOpt;
use url::{Url, ParseError};
use std::collections::HashMap;
use serde_json::Value;

// Define a command-line structure using StructOpt
#[derive(Debug, StructOpt)]
#[structopt(name = "cli-example", about = "An example using structopt")]
struct Cli {
    /// The url to process
    url: String,

    /// Post data
    #[structopt(short = "d", default_value = "")]
    data: String,

    /// Post method used
    #[structopt(short = "X", default_value = "GET")]
    post: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    // println!("{:#?}", args);
    println!("Requesting URL: {}", args.url);
    println!("Method: GET");

    let issue_list_url = Url::parse(&args.url);
    let issue_list_url = match issue_list_url {
        Ok(url) => url,
        Err(err) => {
            // println!("{}", err);
            match err {
                ParseError::RelativeUrlWithoutBase => println!("Error: The URL does not have a valid base protocol."),
                ParseError::InvalidIpv4Address => println!("Error: The URL contains an invalid IPv4 address."),
                ParseError::InvalidIpv6Address => println!("Error: The URL contains an invalid IPv6 address."),
                ParseError::InvalidPort => println!("Error: The URL contains an invalid port number."),
                _ => println!("Other err"),
            }
            
            return Ok(());
        }
    };
    // println!("{}", issue_list_url.scheme());
    // 问题: 都有啥是valid base protocol, 除了https/http/ftp    
    if issue_list_url.scheme() != "https" && issue_list_url.scheme() != "http" {
        println!("Error: The URL does not have a valid base protocol.");
        return Ok(());
    }



    let url = args.url.clone();
    let response = reqwest::get(url).await;
    let response = match response {
        Ok(response) => {
            // println!("{}", response.status());
            response
        },
        Err(err) => {
            // println!("{}", err);
            println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
            
            return Ok(());
        }
    };
    if response.status().is_client_error()  {
        println!("Error: Request failed with status code: 404.");
        return Ok(());
    }

    // Check if the response is JSON formatted
    match response.json::<Value>().await {
        Ok(jb) => {
            // let json_body: serde_json::Value = res.json().await?;
            println!("Response body (JSON with sorted keys):\n{:#?}", jb);  
            return Ok(());
        },
        Err(_) => {
            println!("The response is not valid JSON.");
        }
    }

    // 问题: 如何解决response已经被消耗的问题
    // 方法: 用 header content-type
    let url = args.url.clone();
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    println!("Response body:\n{}", body);

    
       



    
    if args.post == "POST" {
        let url = args.url.clone();
        println!("Requesting URL: {}", args.url);
        println!("Method: {}", args.post);
        println!("Data: {}", args.data);
    
        let mut map = HashMap::new();
        let parts: Vec<&str> = args.data.split('&').collect();
        for part in parts {
            let kv: Vec<&str> = part.split('=').collect();
            map.insert(kv[0], kv[1]);
        }

        
        let client = reqwest::Client::new();
        let res = client.post(url)
            .json(&map)
            .send()
            .await?;


        // let json_body = res.json().await;
        // let json_body: serde_json::Value = match json_body {
        //     Ok(jb) => jb,
        //     Err(err) => {
        //         println!("Err: 333");
        //         // return Ok(());
        //     }
        // };
        // // println!("Response body (JSON with sorted keys):\n{:#?}", json_body);  

        // let body = res.text().await?;
        // println!("Response body:\n{}", body);








    } else {
        // println!("Post method error");
    }

    Ok(())
}
