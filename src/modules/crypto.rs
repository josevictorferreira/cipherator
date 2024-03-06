use openssl::symm::{Cipher, Crypter, Mode};
use rand::{thread_rng, Rng};
use openssl::pkcs5::pbkdf2_hmac;
use openssl::hash::MessageDigest;
use base64::prelude::*;

pub fn encrypt_data(data: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Cipher::aes_128_cbc();
    let mut salt = [0u8; 16];
    thread_rng().fill(&mut salt);
    let iter = 20000;
    let key_len = cipher.key_len();
    let iv_len = cipher.iv_len().unwrap();
    let mut key_iv = vec![0; key_len + iv_len];
    pbkdf2_hmac(password.as_bytes(), &salt, iter, MessageDigest::sha1(), &mut key_iv)?;

    let key = &key_iv[0..key_len];
    let iv = &key_iv[key_len..key_len + iv_len];

    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;
    let mut encrypted_data = vec![0; data.len() + cipher.block_size()];
    let mut count = encrypter.update(data.as_bytes(), &mut encrypted_data)?;
    count += encrypter.finalize(&mut encrypted_data[count..])?;
    encrypted_data.truncate(count);

    let mut encrypted_data_base64 = BASE64_STANDARD.encode(&salt);
    encrypted_data_base64.push_str(&BASE64_STANDARD.encode(&encrypted_data));
    Ok(encrypted_data_base64)
}

// Catch error on decryption
pub fn decrypt_data(data: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed_data = &data.trim_end();
    let cipher = Cipher::aes_128_cbc();
    let salt = match BASE64_STANDARD.decode(&trimmed_data[0..24]) {
        Ok(salt) => salt,
        Err(_) => return Err("Invalid data".into()),
    };
    let encrypted_data = match BASE64_STANDARD.decode(&trimmed_data[24..]) {
        Ok(encrypted_data) => encrypted_data,
        Err(_) => return Err("Invalid data".into()),
    };
    let iter = 20000;
    let key_len = cipher.key_len();
    let iv_len = cipher.iv_len().unwrap();
    let mut key_iv = vec![0; key_len + iv_len];
    match pbkdf2_hmac(password.as_bytes(), &salt, iter, MessageDigest::sha1(), &mut key_iv) {
        Ok(_) => (),
        Err(_) => return Err("Invalid password".into()),
    };

    let key = &key_iv[0..key_len];
    let iv = &key_iv[key_len..key_len + iv_len];

    let mut decrypter = match Crypter::new(cipher, Mode::Decrypt, key, Some(iv)) {
        Ok(decrypter) => decrypter,
        Err(_) => return Err("Invalid data".into()),
    };
    let mut decrypted_data = vec![0; encrypted_data.len() + cipher.block_size()];
    let mut count = match decrypter.update(&encrypted_data, &mut decrypted_data) {
        Ok(count) => count,
        Err(_) => return Err("Invalid data".into()),
    };
    count += match decrypter.finalize(&mut decrypted_data[count..]) {
        Ok(count) => count,
        Err(_) => return Err("Invalid data".into()),
    };
    decrypted_data.truncate(count);
    Ok(String::from_utf8(decrypted_data)?)
}
