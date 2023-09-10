use mokareads_core::awesome_lists::{AwesomeList, Repository};
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // each page contains 30 repositories
    // so let's try to get the first 150
    let pages = 5;
    let awesome_list = AwesomeList::new(10).await?;
    println!("{:?}", awesome_list);
    println!("Count: {}", awesome_list.count_repos());
    Ok(())
}
