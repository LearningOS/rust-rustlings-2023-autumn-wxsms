use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args().nth(1).expect("missing url");

    // let url = "https://www.thepaper.cn/newsDetail_forward_25027317";
    let out = "page.md";

    println!("fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("converting html...");
    let md = html2md::parse_html(&body);

    println!("writing file...");
    fs::write(out, md.as_bytes())?;
    println!("written to file {}", out);

    Ok(())
}
