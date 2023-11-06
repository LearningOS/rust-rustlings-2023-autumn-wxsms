use anyhow::{anyhow, Error, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "httpie")]
#[command(bin_name = "httpie")]
#[command(author = "wxsms@foxmail.com")]
#[command(version = "0.0.1")]
#[command(about = "have fun!")]
struct HttpieCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    /// get request
    Get {
        /// get request url
        #[arg(required = true, value_parser = validate_url)]
        url: String,
    },

    /// post request
    Post {
        /// post request url
        #[arg(required = true, value_parser = validate_url)]
        url: String,
        /// post body
        #[arg(value_parser = validate_kv_pair)]
        body: Vec<KvPair>,
    },
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("failed to parse kv pare: {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).into(),
            v: (split.next().ok_or_else(err)?).into(),
        })
    }
}

fn validate_url(s: &str) -> Result<String> {
    Ok(s.parse::<Url>()?.into())
}

fn validate_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse::<KvPair>()?.into())
}

async fn get(client: Client, url: &str) -> Result<()> {
    let r = client.get(url).send().await?;
    print_result(r).await?;
    Ok(())
}

async fn post(client: Client, url: &str, body: Vec<KvPair>) -> Result<()> {
    let mut b = HashMap::new();
    for p in body.iter() {
        b.insert(&p.k, &p.v);
    }

    let r = client.post(url).json(&b).send().await?;
    print_result(r).await?;
    Ok(())
}

fn print_status(r: &Response) {
    println!("{}\n", format!("{:?} {}", r.version(), r.status()).blue());
}

fn print_headers(r: &Response) {
    for (name, value) in r.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n");
}

fn print_body(m: Option<Mime>, b: &str) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(b).unwrap().cyan())
        }
        _ => println!("{}", b),
    }
}

async fn print_result(r: Response) -> Result<()> {
    print_status(&r);
    print_headers(&r);
    let mime = get_content_type(&r);
    let body = r.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn get_content_type(r: &Response) -> Option<Mime> {
    r.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = HttpieCli::parse();
    let client = Client::new();
    let result = match args.command {
        Commands::Get { url } => get(client, &url).await?,
        Commands::Post { url, body } => post(client, &url, body).await?,
    };
    Ok(result)
}
