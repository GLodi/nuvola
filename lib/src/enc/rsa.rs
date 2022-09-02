use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

pub fn encrypt_file(file: &Vec<u8>) -> Vec<u8> {
    println!("Original:");
    println!("{:?}", std::str::from_utf8(file).unwrap());

    println!("Start encrypt_file");
    let mut rng = rand::thread_rng();

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    println!("Generated keys");

    // Encrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key
        .encrypt(&mut rng, padding, &file[..])
        .expect("failed to encrypt");
    assert_ne!(&file[..], &enc_data[..]);
    println!("Encrypted");
    println!("{enc_data:?}");

    // Decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key
        .decrypt(padding, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&file[..], &dec_data[..]);
    println!("Decrypted");
    println!("{:?}", std::str::from_utf8(&dec_data).unwrap());

    enc_data
}
