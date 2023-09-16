use std::error::Error;

use rocket::tokio;

use mokareads_core::awesome_lists::AwesomeList;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // each page contains 30 repositories
    // so let's try to get the first 300 repositories
    let awesome_list = AwesomeList::new(10).await?;
    println!("{:?}", awesome_list);
    println!("Count: {}", awesome_list.count_repos());
    Ok(())
}
