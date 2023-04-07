use async_std::io::prelude::*;
use async_std::net;

// This is a synchronous function that calls an asynchronous function.
// block_on blocks the entire thread until value is returned from the future.
// cheapo_request is async - that means when .await is called on any of it's sub-futures,
// block_on goes to sleep until some future is ready to be polled again.
pub fn call_from_sync() -> std::io::Result<()> {
    use async_std::task;

    let response = task::block_on(cheapo_request("wikipedia.org", 80, "/"))?;
    println!("{}", response);
    Ok(())
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
