use std::{collections::HashMap, fs::OpenOptions, io::Write};
fn string_seq_factory() -> impl FnMut() -> String {
    let mut current = vec![b'a'];
    move || {
        let s = String::from_utf8(current.clone()).unwrap();
        for i in (0..current.len()).rev() {
            if current[i] < b'z' {
                current[i] += 1;
                return s;
            }
            current[i] = b'a';
        }
        current.insert(0, b'a');
        s
    }
}
fn main() {
    let mut stringseq = string_seq_factory();
    let mut x: HashMap<u64, String> = HashMap::new();
    for y in 1..=100 {
        x.insert(y, stringseq());
    }
    let t = serde_json::ser::to_string(&x);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("db.json")
        .expect("Unable to open or create the file");
    let _ = file.write(t.unwrap().as_bytes());
}
