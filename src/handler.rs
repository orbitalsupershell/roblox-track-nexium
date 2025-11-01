use crate::basicserver;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
fn e404(s: &mut TcpStream) {
    let _ = s.write("HTTP/1.1 404 NOT FOUND\r\ncontent-type: text/html\r\n\r\n".as_bytes());
    let _ = s.write_all(include_str!("static/404.html").as_bytes());
}
fn e500(s: &mut TcpStream) {
    let _ =
        s.write("HTTP/1.1 500 Internal Server Error\r\ncontent-type: text/html\r\n\r\n".as_bytes());
    let _ = s.write_all(include_str!("static/500.html").as_bytes());
}
pub fn handle(
    stream: &mut TcpStream,
    presencelist: Arc<RwLock<HashMap<u64, (String, String, u8)>>>,
) {
    let a = basicserver::stream_read(stream);
    let location = basicserver::extract_location(&a[0]);
    println!("Request to {location}",);

    match location {
        "/" => {
            let _ = stream.write("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".as_bytes());
            let _ = stream.write_all(include_bytes!("static/index.html"));
        }
        "/api/trackingupdate" => {
            let x = presencelist.read().unwrap();
            let y = serde_json::to_string(&*x);
            match y {
                Ok(y) => {
                    let _ = stream
                        .write("HTTP/1.1 200 OK\r\nContent-Type: text/json\r\n\r\n".as_bytes());
                    let _ = stream.write_all(y.as_bytes());
                    return;
                }
                Err(_) => {
                    e500(stream);
                    return;
                }
            }
        }
        "/logo.png" => {
            let _ = stream.write("HTTP/1.1 200 OK\r\nContent-Type: image/png\r\n\r\n".as_bytes());
            let _ = stream.write_all(include_bytes!("static/logo.png"));
        }
        _ => {
            e404(stream);
            return;
        }
    }
}
