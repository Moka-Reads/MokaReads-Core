use std::thread::sleep;
use std::time::Duration;

use rocket::tokio;

use mokareads_core::awesome_lists::AwesomeList;
use mokareads_core::Result;

const PAGES: [usize; 4] = [10, 20, 30, 40];

async fn iter_pages(start: usize, end: usize) -> Result<()> {
    for p in start..=end {
        let _al = AwesomeList::new(p).await?;
        sleep(Duration::from_secs(10));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut start = 1;
    for page in PAGES {
        match iter_pages(start, page).await {
            Ok(_) => start = page,
            Err(_) => {
                println!("Broke at {page}");
                break;
            }
        };
    }

    Ok(())
}
