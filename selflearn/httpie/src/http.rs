use crate::kv_pair;
use anyhow::Result;
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response};
use std::collections::HashMap;

pub async fn get(client: Client, url: &str) -> Result<()> {
    let r = client.get(url).send().await?;
    print_result(r).await?;
    Ok(())
}

pub async fn post(client: Client, url: &str, body: Vec<kv_pair::KvPair>) -> Result<()> {
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
