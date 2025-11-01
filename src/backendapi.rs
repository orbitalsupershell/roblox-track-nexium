use reqwest::{blocking, header::CONTENT_TYPE};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

fn create_client() -> reqwest::blocking::Client {
    let api_client = blocking::ClientBuilder::new()
        .build()
        .expect("Failure to build client");
    return api_client;
}

fn resolve_names(people: &Vec<u64>) -> HashMap<u64, String> {
    let api_client = create_client();

    let mut predicate: HashMap<u64, String> = HashMap::new();

    let chunkies: Vec<Vec<u64>> = people.chunks(32).map(|chunk| chunk.to_vec()).collect();
    let mut chunks_done = 0;
    let chunks = chunkies.len();
    for x in chunkies {
        let data = serde_json::to_string(&x).expect("serialize failed");

        let t = api_client
            .post("https://users.roblox.com/v1/users")
            .header(CONTENT_TYPE, "text/json")
            .body(format!("{{\"userIds\":{}}}", data));

        match t.send() {
            Ok(a) => {
                let t = a.text().unwrap();
                let x: Value = serde_json::from_str(&t).unwrap();

                if let Some(y) = x["data"].as_array() {
                    for item in y {
                        let id = item["id"].as_u64();
                        let name = item["name"].as_str();

                        if id.is_some() && name.is_some() {
                            predicate.insert(id.unwrap(), name.unwrap().to_owned());
                        }
                    }
                }
            }
            Err(a) => {
                eprintln!("{:?},{:?}", a.to_string(), a.status())
            }
        }
        chunks_done = chunks_done + 1;
        // println!("{chunks_done}/{chunks}");
        if chunks_done == chunks {
        } else {
            sleep(Duration::from_millis(2000));
        }
    }

    return predicate;
}

fn resolve_presence(people: &Vec<u64>) -> HashMap<u64, u8> {
    let api_client = create_client();
    let mut predicate: HashMap<u64, u8> = HashMap::new();

    let chunkies: Vec<Vec<u64>> = people.chunks(32).map(|chunk| chunk.to_vec()).collect();
    let mut chunks_done = 0;
    let chunks = chunkies.len();
    for x in chunkies {
        let data = serde_json::to_string(&x).expect("serialize failed");

        let t = api_client
            .post("https://presence.roblox.com/v1/presence/users")
            .header(CONTENT_TYPE, "text/json")
            .body(format!("{{\"userIds\":{}}}", data));

        match t.send() {
            Ok(a) => {
                let t = a.text().unwrap();
                let x: Value = serde_json::from_str(&t).unwrap();

                if let Some(y) = x["userPresences"].as_array() {
                    for item in y {
                        let id = item["userId"].as_u64();
                        let presence = item["userPresenceType"].as_u64();

                        if id.is_some() && presence.is_some() {
                            predicate.insert(id.unwrap(), presence.unwrap() as u8);
                        }
                    }
                }
            }
            Err(a) => {
                eprintln!("{:?},{:?}", a.to_string(), a.status());
            }
        }
        chunks_done = chunks_done + 1;
        if chunks == chunks_done {
        } else {
            sleep(Duration::from_millis(2000));
        }
    }

    return predicate;
}

pub fn main(sharedmap: Arc<RwLock<HashMap<u64, (String, String, u8)>>>) {
    let file = fs::read_to_string("./db.json").expect("\n\n!!!Could not load ./db.json, please ensure it exists!!! hint (cargo run --bin regenerate_database)\n\n\n");

    let db: HashMap<u64, String> = serde_json::from_str(&file).unwrap();
    let mut people = Vec::new();
    let targets = {
        for (k, _) in &db {
            people.push(*k)
        }
        resolve_names(&people)
    };
    let mut sharedmap_rwlock = sharedmap.write().expect("oh no");
    for (id, name) in targets {
        let faction = &db.get(&id).unwrap();
        sharedmap_rwlock.insert(id, (name, faction.to_string(), 0));
    }
    drop(sharedmap_rwlock);

    loop {
        {
            let presence = resolve_presence(&people);

            let mut sharedmap_rwlock = sharedmap.write().expect("oh no");
            for (id, prectype) in presence {
                match sharedmap_rwlock.get_mut(&id) {
                    Some(v) => {
                        v.2 = prectype;
                    }
                    None => {}
                };
            }
            println!("Refreshed");
            drop(sharedmap_rwlock);
        }
        sleep(Duration::from_secs(60));
    }
}
