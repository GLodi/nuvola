use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn read(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    for value in &buffer {
        println!("BYTE: {}", value);
    }

    Ok(buffer)
}

pub fn write(path: &str, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, data).unwrap();
    Ok(())
}
