use argon2::Argon2;
use rand::Rng;
use std::{collections::HashMap, fs::OpenOptions};
use std::{
    io::{Read, Write},
    ops::Index,
};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

use crate::input;

fn generate_salt(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

fn generate_key(password: String, salt: &Vec<u8>) -> [u8; 32] {
    let mut key = [0u8; 32]; // Can be any desired size
    match Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key) {
        Ok(data) => println!("{data:?}"),
        Err(error) => println!("{error:?}"),
    }
    key
}
// type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn encrypt(key: [u8; 32], message: String) -> Vec<u8> {
    let key_array = Key::<Aes256Gcm>::from_slice(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key_array);

    let ciphered_data = cipher
        .encrypt(&nonce, message.as_bytes())
        .expect("failed to encrypt");
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    encrypted_data
}

fn decrypt(key: [u8; 32], encrypted_data: Vec<u8>) -> Result<String, String> {
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .map_err(|_| "Invalid credentials".to_owned())?;
    String::from_utf8(plaintext).map_err(|_| "The vault is corrupted".to_owned())
}

#[derive(Clone)]
pub(crate) struct Vault {
    pub(crate) name: String,
    key: [u8; 32],
    credentials: HashMap<String, String>,
}

impl Vault {
    /// Create a new Vault
    ///
    /// the `new` method creates a new instance of `Vault` with the given name
    /// and an empty list of credentials
    pub(crate) fn new(name: String, key: [u8; 32]) -> Self {
        Self {
            name,
            key,
            credentials: HashMap::new(),
        }
    }

    /// Open an existing vault
    ///
    /// If a vault exists, it opens the vault, else shows an error message saying it doesn't exist.
    pub(crate) fn open(name: String, password: String) -> Result<Self, String> {
        match OpenOptions::new()
            .read(true)
            .create_new(false) // This will cause an error if the file already exists
            .open(format!("{name}.vault"))
            .as_mut()
        {
            Ok(file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                let data: Vec<String> = buf.split("\n").map(|s| s.to_string()).collect();

                let salt_string = data.index(0);
                let _password = data.index(1);
                let _creds = data.index(2);

                let salt = hex::decode(salt_string.clone().to_owned()).unwrap();

                let key = generate_key(password.clone(), &salt);
                decrypt(key, hex::decode(_password).unwrap())?;
                let vault = Self::new(name, key);
                Ok(vault)
            }
            Err(_) => Err("Could not open the vault".to_string()),
        }
    }

    /// Create
    ///
    /// This method creates a new vault with credentials.
    /// The vault contains encrypted credentials along with few encryption data
    /// such as number of iterations and salt.
    pub(crate) fn create(name: String) {
        match OpenOptions::new()
            .write(true)
            .create_new(true) // This will cause an error if the file already exists
            .open(format!("{name}.vault"))
            .as_mut()
        {
            Ok(file) => {
                let password = input("Enter password: ");
                let salt = generate_salt(20);
                let key = generate_key(password.clone(), &salt);

                println!("The vault \"{name}\" has been successfully created");
                let mut data = vec![];
                data.push(hex::encode(salt.clone()));
                data.push(hex::encode(encrypt(key, password)));
                data.push("".to_string());
                let _ = file.write(data.join("\n").as_bytes());
            }
            Err(e) => {
                println!("Could not create the vault \"{name}\"");
                println!("{e}");
            }
        }
    }
}
