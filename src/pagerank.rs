use crate::url::Url;
use std::collections::HashMap;
extern crate rand;



pub fn pagerank(urls: &mut Vec<Url>, iterations: usize, damping_factor: f64) {
  let num_urls = urls.len() as f64;
  let initial_value = 1.0 / num_urls;

  // Initialize PageRank values
  for url in urls.iter_mut() {
      url.pagerank = initial_value;
  }

  // Create a HashMap for easy access to URLs by their links
  let mut url_map: HashMap<String, usize> = HashMap::new();
  for (index, url) in urls.iter().enumerate() {
      url_map.insert(url.link.clone(), index);
  }

  for _ in 0..iterations {
      let mut new_pageranks = vec![0.0; urls.len()];

      for (_, url) in urls.iter().enumerate() {
          let outgoing_links = url.outlinks.len() as f64;

          if outgoing_links > 0.0 {
              let rank_to_distribute = url.pagerank / outgoing_links;
              for outlink in &url.outlinks {
                  if let Some(&index) = url_map.get(&outlink.clone()) {
                      new_pageranks[index] += rank_to_distribute;
                  }
              }
          } else {
              // If a page has no outlinks, distribute its rank evenly to all pages
              let rank_to_distribute = url.pagerank / num_urls;
              for pr in new_pageranks.iter_mut() {
                  *pr += rank_to_distribute;
              }
          }
      }

      // Apply damping factor and update PageRank values
      for (i, url) in urls.iter_mut().enumerate() {
          url.pagerank = (1.0 - damping_factor) / num_urls + damping_factor * new_pageranks[i];
      }
  }
}