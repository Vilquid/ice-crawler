use std::env;

use super::schema::{users};
use aes_gcm::aead::OsRng;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use argonautica::{Hasher, Verifier};
use argon2::{self, Config};
use sha3::{Digest, Sha3_512};
use aes_gcm::{Key};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{Rng, thread_rng};
use generic_array::{GenericArray, sequence::GenericSequence};
use generic_array::typenum::{U12, U32, U48, U256};
use crypto::symmetriccipher::BlockEncryptor;
use rand::RngCore;
use base64;
use base64::Engine;
use base64::engine::general_purpose;
use nom::AsBytes;


#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn dechiffrement(password_browser: String, mut password_db: String) -> bool {
    let mut key_bytes = env::var("SECRET_KEY").unwrap();
    let mut nonce_bytes = env::var("NONCE").unwrap();
    let nonce = Nonce::from_slice(nonce_bytes.as_bytes());


    let decoded_ciphertext = general_purpose::STANDARD.decode(password_db).unwrap();
    let cipher = Aes256Gcm::new_from_slice(key_bytes.as_bytes()).unwrap();
    let plaintext = cipher.decrypt(&nonce, decoded_ciphertext.as_bytes()).unwrap();

    password_db = String::from_utf8(plaintext).unwrap();

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(password_db)
        .with_password(password_browser)
        .with_secret_key(key_bytes)
        .verify()
        .unwrap();

    is_valid
}


impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        dotenv().ok();

        // Étape 1 : Salage et hachage du mot de passe avec Argonautica
        let secret = env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set");

        let hash = Hasher::default()
            .with_password(password)
            .with_secret_key(secret)
            .hash()
            .unwrap();

        // Création de la clé de chiffrement

        // Création d'un nonce unique
        //let nonce = rng.gen();
        let mut nonce_bytes = env::var("NONCE").unwrap();
        let nonce = Nonce::from_slice(nonce_bytes.as_bytes());


        // Chiffrement du hash avec AES-GCM
        let mut key_bytes = env::var("SECRET_KEY").unwrap();
        let cipher = Aes256Gcm::new_from_slice(key_bytes.as_bytes()).unwrap();
        let ciphertext = cipher.encrypt(&nonce, hash.as_bytes()).unwrap();

        // Encodage du ciphertext en base64 pour le stockage
        let encoded_ciphertext = general_purpose::STANDARD.encode(ciphertext);

        NewUser {
            username: username,
            email: email,
            password: encoded_ciphertext.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}