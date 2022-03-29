use sha2::{Digest, Sha256};

pub fn print(data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    println!("file hash: {:?}", &hash);
    Ok(())
}
