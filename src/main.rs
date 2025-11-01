use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use nexium::backendapi;
use nexium::basicserver;
use nexium::handler;

fn main() {
    let presencemap = HashMap::new();
    let presencemap_arc: Arc<RwLock<HashMap<u64, (String, String, u8)>>> =
        Arc::new(RwLock::new(presencemap));

    println!("Server start");
    let backendmap = presencemap_arc.clone();
    let th = thread::spawn(move || {
        backendapi::main(backendmap.clone());
    });

    let precmap = presencemap_arc.clone();
    basicserver::serve(
        8080,
        precmap,
        {
            move |stream, precmap| {
                handler::handle(stream, precmap);
            }
        },
        16,
    );
    let _ = th.join();
}
