use crate::pool::Pool;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::{
    io::{BufRead, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};
pub fn serve<
    T: FnOnce(&mut TcpStream, Arc<RwLock<HashMap<u64, (String, String, u8)>>>)
        + Send
        + Clone
        + Copy
        + 'static,
>(
    port: u16,
    precmap: Arc<RwLock<HashMap<u64, (String, String, u8)>>>,
    handler: T,
    threads: usize,
) {
    let listener =
        TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).expect("Could not bind to port");
    let pool = Pool::new(threads);
    for stream in listener.incoming() {
        let pm = precmap.clone();
        pool.execute(move || match stream {
            Ok(mut stream) => handler(&mut stream, pm),
            Err(_) => {}
        })
    }
}
pub fn stream_read(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(a) => a,
            Err(_) => "".to_string(),
        })
        .take_while(|line| !line.is_empty())
        .collect();
    if http_request.len() == 0 {
        return vec!["".to_string()];
    }
    return http_request;
}
pub fn extract_location(request: &str) -> &str {
    let parts: Vec<&str> = request.split_whitespace().collect();
    if parts.len() >= 2 { parts[1] } else { "" }
}
