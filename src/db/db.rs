use std::collections::HashMap;

extern crate rustbreak;
use rustbreak::{deser::Ron, MemoryDatabase};

fn main() -> rustbreak::Result<()> {
    let db = MemoryDatabase::<HashMap<u32, String>, Ron>::memory(HashMap::new())?;

    println!("Writing to Database");
    db.write(|db| {
        db.insert(0, String::from("world"));
        db.insert(1, String::from("bar"));
    });

    db.read(|db| {
        // db.insert("foo".into(), String::from("bar"));
        // The above line will not compile since we are only reading
        println!("Hello: {:?}", db.get(&0));
    })?;

    Ok(())
}
