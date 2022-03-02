use sha2::{Digest, Sha256};
use std::fs::File;

pub fn print(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    println!("file hash: {:?}", &hash);
    Ok(())
}
