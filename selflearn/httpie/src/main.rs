use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::{Client, Url};

mod http;
mod kv_pair;

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
        body: Vec<kv_pair::KvPair>,
    },
}

fn validate_url(s: &str) -> Result<String> {
    Ok(s.parse::<Url>()?.into())
}

fn validate_kv_pair(s: &str) -> Result<kv_pair::KvPair> {
    Ok(s.parse::<kv_pair::KvPair>()?.into())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = HttpieCli::parse();
    let client = Client::new();
    let result = match args.command {
        Commands::Get { url } => http::get(client, &url).await?,
        Commands::Post { url, body } => http::post(client, &url, body).await?,
    };
    Ok(result)
}
