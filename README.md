## Pagerank microservice in Rust

Runs as a dockerised microservice, refreshes page-rank scores every minute. By checking the existence of new URLs. 
If new URLs are present page rank runs on all pages.

$$PR(A)=\frac{1-d}{N} + d\sum_{i=1}^{n}\frac{PR(L_i)}{C(L_i)}$$

where $PR(A)$ is the  PageRank of page $A$, $d$ is the damping factor (usually set around $0.85$), $N$ is the total number of pages, $PR(L_i)$ is the PageRank of page $L_i$ linking to page $A$, and $C(L_i)$ is the number of outbound links on page $L_i$

`port used: 6379`

To use this container, first build the image, and then create a docker virtual network. 

```console
docker network create my_network
docker run -d --name redis --network my_network redis

docker build -t pagerank .
docker run --rm --network my_network pagerank
```

Note a container must be running redis before running this. To run redis:

```console

docker pull redis
docker run -d --name redis redis
docker exec -it my-redis redis-cli
```

After this the Redis Cli will open. Start by inserting some initial hardcoded values:

```
SADD urls "https://www.google.com" "https://www.facebook.com" "https://www.twitter.com" "https://www.youtube.com" "https://www.instagram.com"
SET "https://www.google.com" '{"link":"https://www.google.com","pagerank":0.0,"outlinks":[],"inlinks":[]}'
SET "https://www.facebook.com" '{"link":"https://www.facebook.com","pagerank":0.0,"outlinks":[],"inlinks":[]}'
SET "https://www.twitter.com" '{"link":"https://www.twitter.com","pagerank":0.0,"outlinks":[],"inlinks":[]}'
SET "https://www.youtube.com" '{"link":"https://www.youtube.com","pagerank":0.0,"outlinks":[],"inlinks":[]}'
SET "https://www.instagram.com" '{"link":"https://www.instagram.com","pagerank":0.0,"outlinks":[],"inlinks":[]}'
SET new_urls 1
```

URL insertion involves 3 operations:

```
SADD <URL>
SET  <URL> <Value>
SET  new_urls 1
```

For example,

```
SADD "https://www.example.com"
SET  "https://www.example.com" {"link":"https://www.example.com","pagerank":0.0,"outlinks":["https://www.google.com"],"inlinks":["https://www.facebook.com"]}'
SET new_urls 1
```

Note: new_urls is reset every minute when pagerank runs, consuming all newly added pages.

I know there's no need to run pagerank on all new pages.