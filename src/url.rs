use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub link: String,
    pub pagerank: f64,
    pub outlinks: Vec<String>,
    pub inlinks: Vec<String>,
}

impl Url {
    pub fn new(link: String) -> Url {
        Url {
            link,
            pagerank: 0.0,
            outlinks: Vec::new(),
            inlinks: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!();
        println!("Url: {}", self.link);
        println!("\tPageRank: {}", self.pagerank);
        println!("\tOutlinks: ");
        for outlink in &self.outlinks {
            println!("\t\t{}", outlink);
        }
        println!("\tInlinks: ");
        for inlink in &self.inlinks {
            println!("\t\t{}", inlink);
        }
    }
}

