extern crate redis;

mod pagerank;
mod url;
mod utils;
use std::error::Error;
use utils::RedisClient;


pub fn main() -> Result<(), Box<dyn Error>> {
    let mut redis_client = RedisClient::new()?;

    loop {
        if redis_client.check_new_urls()? {
            let mut urls = redis_client.get_all_urls()?;
            pagerank::pagerank(&mut urls, 100, 0.85);

            for url in urls {
                redis_client.update_url(&url)?;
            }

            println!("PageRank updated for all URLs");
        }

        // Sleep for a while before checking again
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

