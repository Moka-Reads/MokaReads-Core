use std::io::stdin;

use rocket::tokio;

use mokareads_core::api::Api;
use mokareads_core::resources::{Cacher, Searcher};
use mokareads_core::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let data = Api::Resources.get().await?;
    let cacher: Cacher = serde_json::from_str(&data).unwrap();
    let searcher = Searcher::new(&cacher);
    //let str = serde_json::to_string_pretty(&searcher).unwrap();
    //println!("{}", str);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let result = searcher.search(input.trim().to_string());
    println!("{:?}", result);
    Ok(())
}