use error_chain::error_chain;
use reqwest;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
fn main() -> Result<()> {
    let res = reqwest::blocking::get("https://httpbin.org/get")?;
    println!("{:?}", res.status());
    Ok(())
}
