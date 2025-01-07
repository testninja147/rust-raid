use aes::{
    cipher::{
        block_padding::{Iso10126, Pkcs7},
        generic_array::GenericArray,
        BlockDecrypt, BlockDecryptMut, BlockEncrypt, BlockEncryptMut, KeyInit, KeyIvInit,
    },
    Aes256,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use clap::builder::Str;
use rand::Rng;
use std::{collections::HashMap, fs::OpenOptions};
use std::{
    io::{Read, Write},
    ops::Index,
};

use crate::input;

// define encoder and decoder types
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
}

fn hex_string_to_bytes(string: String) -> Result<Vec<u8>, String> {
    let _str = string.as_str();

    // u8 hex string should always have even length
    if _str.len() % 2 != 0 {
        return Err("Hex string has an odd length".to_owned());
    }

    // let mut data = vec![];
    let data = (0.._str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&_str[i..i + 2], 16).unwrap())
        .collect();
    Ok(data)
}
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

fn encrypt(key: [u8; 32], message: String) -> Vec<u8> {
    let key = GenericArray::from(key);
    let mut message = message.as_bytes().to_vec();
    let len = message.len();
    let cipher = Aes256::new(&key);
    cipher
        .encrypt_padded_mut::<Iso10126>(&mut message, len)
        .unwrap();
    println!("{:?}", message);
    message
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

                let salt = hex_string_to_bytes(salt_string.clone().to_owned()).unwrap();
                println!("salt: {salt:?}");
                println!("pass: {_password:?}");
                println!("data: {_creds:?}");

                let key = generate_key(password, &salt);
                let vault = Self::new(name, key);
                Ok(vault)
            }
            Err(_) => {
                // println!("Could not create the vault \"{name}\"");
                // println!("{e}");
                Err("Could not open the vault".to_string())
            }
        }

        // vault

        // let salt = generate_salt(20);
        // let key = generate_key(password, &salt);
        // let key_string: String = key.iter().map(|k| return format!("{k:02x}")).collect();
        // println!("key_string: {:?}", key_string);

        // let encrypted = encrypt(key, String::from("Test data 111111111111111"));
        // println!("Encrypted: {:?}", encrypted);

        // let vault = Self::new(name, key);
        // vault
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
                data.push(bytes_to_hex_string(salt.clone()));
                data.push(bytes_to_hex_string(salt.clone()));
                // data.push(bytes_to_hex_string(encrypt(key, password)));
                data.push("".to_string());

                let _ = file.write(data.join("\n").as_bytes());

                // let _ = file.write(bytes_to_hex_string(salt).as_bytes());
                // let _ = file.write(b"\n");
                // let _ = file.wri
                // let _ = file.writ
            }
            Err(e) => {
                println!("Could not create the vault \"{name}\"");
                println!("{e}");
            }
        }
    }
}

// [140, 236, 35, 129, 174, 152, 176, 176, 85, 233, 122, 112, 21, 31, 157, 154, 64, 3, 98, 45, 151, 192, 106, 228, 190, 132, 161, 162, 99, 125, 13, 232]
