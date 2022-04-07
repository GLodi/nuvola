use reed_solomon_erasure::galois_8::ReedSolomon;

pub fn encode(data: &Vec<u8>) -> Vec<u8> {
    let chunk_length = 4;
    let data_chunk_amount = data.chunks(chunk_length).into_iter().count();
    let parity_chunk_amount = 3;

    let r = ReedSolomon::new(data_chunk_amount, parity_chunk_amount).unwrap(); // 3 data shards, 2 parity shards

    let mut master_copy = vec![];

    for chunk in data.chunks(chunk_length) {
        let mut c = chunk.to_vec();
        c.resize(chunk_length, 0);
        master_copy.push(c);
    }

    for _ in 0..parity_chunk_amount {
        master_copy.push(vec![0; chunk_length]);
    }

    println!("before encoding:");
    println!("{master_copy:?}");
    println!();

    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    let encoded = master_copy.clone().into_iter().flatten().collect();

    println!("encoded:");
    println!("{master_copy:?}");
    println!();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

    // We can remove up to 2 shards, which may be data or parity shards
    for i in 0..parity_chunk_amount {
        shards[i] = None;
    }

    println!("after data loss:");
    println!("{shards:?}");
    println!();

    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();

    println!("reconstructed:");
    println!("{shards:?}");
    println!();

    // Convert back to normal shard arrangement
    let reconstructed: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    assert!(r.verify(&reconstructed).unwrap());
    assert_eq!(master_copy, reconstructed);

    encoded
}
