use std::{fs, io::{self, Write}};
use anyhow::Result;
use rand::{thread_rng, RngCore, Rng};
use rand::distributions::Alphanumeric;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::ChaCha20Poly1305;

type HmacSha256 = Hmac<Sha256>;

fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn encrypt1(filename: &str) -> Result<()> {
    let plaintext = fs::read(filename)?;

    let mut key = vec![0u8; 32];
    let password = random_string(90);
    let salt = random_string(70);

    pbkdf2::<HmacSha256>(password.as_bytes(), salt.as_bytes(), 200_000, &mut key);

    let mut iv = [0u8; 12];
    thread_rng().fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    let key_ga = GenericArray::from_slice(&key);
    let cipher = Aes256Gcm::new(key_ga);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .expect("encryption failed");

    let mut file = fs::File::create(filename)?;
    file.write_all(&ciphertext)?;

    println!("1 Successfully AES-GCM");
    Ok(())
}

pub fn encrypt2(filename: &str) -> Result<()> {
    let plaintext = fs::read(filename)?;

    println!("ランダムな文字列を入力してください。長ければ長いほど暗号強度は上がります。\nInput Random String:");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();

    let mut key = vec![0u8; 32];
    let salt = random_string(1247);

    pbkdf2::<HmacSha256>(inp.trim().as_bytes(), salt.as_bytes(), 200_000, &mut key);

    let mut iv = [0u8; 12];
    thread_rng().fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    let key_ga = GenericArray::from_slice(&key);
    let cipher = Aes256Gcm::new(key_ga);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .expect("encryption failed");

    let mut file = fs::File::create(filename)?;
    file.write_all(&ciphertext)?;

    println!("2 Successfully");
    Ok(())
}

pub fn encrypt3(filename: &str) -> Result<&str> {
    let plaintext = fs::read(filename)?;

    println!("ランダムな文字列を入力してください。長ければ長いほど暗号強度は上がります。\nInput Random String:");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    let password = inp.trim();

    let mut key = [0u8; 32];
    let salt = random_string(16);

    pbkdf2::<HmacSha256>(password.as_bytes(), salt.as_bytes(), 200_000, &mut key);

    let key_ga = GenericArray::from_slice(&key);
    let cipher = ChaCha20Poly1305::new(key_ga);

    let mut iv = [0u8; 12];
    thread_rng().fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .expect("encryption failed");

    let mut file = fs::File::create(filename)?;
    file.write_all(&ciphertext)?;

    println!("3 Successfully (ChaCha20)");
    Ok(filename)
}

pub fn crpy(filename: &str) -> Result<&str> {
    encrypt1(filename)?;
    encrypt2(filename)?;
    encrypt3(filename)
}
