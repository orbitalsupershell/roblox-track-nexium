use std::{
    fs::{self, File},
    io::Read,
};

pub fn opendb(filename: &str) -> String {
    let mut data = String::new();
    let mut db = File::open(filename).expect("Failure to open file");
    fs::File::read_to_string(&mut db, &mut data).expect("Failure to read database to string");
    return data;
}

pub mod backendapi;
pub mod basicserver;
pub mod handler;
pub mod pool;
