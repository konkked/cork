mod store;

use clap::{Command, Arg};
use warp::Filter;
use std::sync::Arc;
use std::net::{SocketAddr, Ipv4Addr};
use log::{info};
use store::{KvStore, Value}; 
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddrV4;

#[derive(Debug, Deserialize)]
struct KeyQuery {
    key: String,
}

#[tokio::main]
async fn main() {
    // Set up command-line arguments
    let matches = Command::new("Key-Value Store")
            .version("1.0")
            .author("Author Name")
            .about("A key-value store")
            .arg(Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the port to listen on")
                .default_value("3030")
                .value_parser(clap::value_parser!(u16)))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging"))
            .get_matches();

    // Get the values of the arguments
    // Configure logging
    if matches.contains_id("verbose") {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    } else {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();
    }

    // Retrieve and parse the port argument
    let port = matches.get_one::<u16>("port")
        .expect("Port value is required");

    info!("Starting server on port {}", port);

    let kv_store = Arc::new(KvStore::new());

    let kv_store_clone = kv_store.clone();
    let set = warp::path("set")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |value: Value| {
            kv_store_clone.set(value);
            warp::reply::with_status("Value set", warp::http::StatusCode::OK)
        });

    let kv_store_clone = kv_store.clone();
    
    let get = warp::path("get")
    .and(warp::get())
    .and(warp::query::<KeyQuery>()) // Assuming you are using query parameters for the key
    .map(move |query: KeyQuery| {
        let response = match kv_store_clone.get(&query.key) {
            Some(value) => warp::reply::json(&json!({ "value": value })),
            None => warp::reply::json(&json!({ "error": "Not found" })),
        };
        
        if kv_store_clone.get(&query.key).is_some() {
            // Return 200 OK if the key is found
            warp::reply::with_status(response, warp::http::StatusCode::OK)
        } else {
            // Return 404 Not Found if the key is not found
            warp::reply::with_status(response, warp::http::StatusCode::NOT_FOUND)
        }
    });

    let kv_store_clone = kv_store.clone();
    let remove = warp::path("remove")
        .and(warp::delete())
        .and(warp::query::<KeyQuery>()) 
        .map(move |query: KeyQuery| {
            kv_store_clone.remove(&query.key);
            warp::reply::with_status("Value removed", warp::http::StatusCode::OK)
        });

    let routes = set.or(get).or(remove);
    let port = *matches.get_one::<u16>("port").expect("Port value is required");
    let addr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port).into();
    info!("Starting server on port {}", port);
    warp::serve(routes).run(addr).await;
}