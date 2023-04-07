use std::vec;

use async_std::io::prelude::*;
use async_std::net;

pub fn run() {
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
        ("seznam.cz".to_string(), 80, "/".to_string()),
    ];

    // This block_on blocks the entire thread, but because many_requests uses task::spawn_local,
    // it runs all three requests concurrently.
    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    use async_std::task;
    let mut handles = vec![];

    for (host, port, path) in requests {
        // Future needs to own the parameters, because it needs to live longer than the for loop
        handles.push(task::spawn_local(cheapo_owning_request(host, port, path)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

async fn cheapo_owning_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, port, &path).await
}

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost:{}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;

    socket.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();

    socket.read_to_string(&mut response).await?;

    Ok(response)
}
