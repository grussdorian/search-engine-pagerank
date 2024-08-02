use crate::url::Url;
use std::error::Error;
use redis::Connection;
use redis::Commands;
use redis::Client;


pub struct RedisClient {
  conn: Connection,
}

impl RedisClient {
  pub fn new() -> Result<Self, redis::RedisError> {
      let client = Client::open("redis://redis:6379")?;
      let conn = client.get_connection()?;
      Ok(RedisClient { conn })
  }

  pub fn get_all_urls(&mut self) -> Result<Vec<Url>, Box<dyn Error>> {
      let urls: Vec<String> = self.conn.smembers("urls")?;
      let mut result = Vec::new();
      for url in urls {
          let json: String = self.conn.get(&url)?;
          let url_struct: Url = serde_json::from_str(&json)?;
          result.push(url_struct);
      }
      Ok(result)
  }

  pub fn add_url(&mut self, url: &Url) -> Result<(), Box<dyn Error>> {
      let json = serde_json::to_string(url)?;
      self.conn.sadd("urls", &url.link)?;
      self.conn.set(&url.link, json)?;
      Ok(())
  }

  pub fn update_url(&mut self, url: &Url) -> Result<(), Box<dyn Error>> {
      let json = serde_json::to_string(url)?;
      self.conn.set(&url.link, json)?;
      Ok(())
  }

  pub fn check_new_urls(&mut self) -> Result<bool, Box<dyn Error>> {
      let new_urls: bool = self.conn.exists("new_urls")?;
      if new_urls {
          self.conn.del("new_urls")?;
      }
      Ok(new_urls)
  }
}
