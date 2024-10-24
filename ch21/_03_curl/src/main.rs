use std::collections::HashMap;
use url::{Url, ParseError};
use url::{Host, Position};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "v", long = "velocity", default_value = "42")]
    speed: f64,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET request
    // let resp = reqwest::get("https://www.eecg.toronto.edu/~bli/ece1724/assignments/files/lab3.html")
    //     .await?
    //     .text()
    //     .await?;
    // println!("{resp:#?}");
    // POST request
    // let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()
    //     .await?;



    // assert!(Url::parse("http://[:::1]") == Err(ParseError::InvalidIpv6Address));
    // let issue_list_url = Url::parse(
    //     "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    // )?;
    // assert!(issue_list_url.scheme() == "https");
    // assert!(issue_list_url.username() == "");
    // assert!(issue_list_url.password() == None);
    // assert!(issue_list_url.host_str() == Some("github.com"));
    // assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
    // assert!(issue_list_url.port() == None);
    // assert!(issue_list_url.path() == "/rust-lang/rust/issues");
    // assert!(issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
    //         Some(vec!["rust-lang", "rust", "issues"]));
    // assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
    // assert!(&issue_list_url[Position::BeforePath..] == "/rust-lang/rust/issues?labels=E-easy&state=open");
    // assert!(issue_list_url.fragment() == None);
    // assert!(!issue_list_url.cannot_be_a_base());



    let opt = Opt::from_args();
    println!("{:?}", opt);



    Ok(())
}


