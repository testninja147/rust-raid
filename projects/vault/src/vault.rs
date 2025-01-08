use argon2::Argon2;
use rand::Rng;
use rpassword::prompt_password;
use std::io::{Read, Write};
use std::{collections::HashMap, fs::OpenOptions};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

use crate::message_box;

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
    salt: Vec<u8>,
    key: [u8; 32],
    credentials: HashMap<String, Vec<u8>>,
}

impl Vault {
    /// Create a new Vault
    ///
    /// the `new` method creates a new instance of `Vault` with the given name
    /// and an empty list of credentials
    pub(crate) fn new(name: String, key: [u8; 32], salt: Vec<u8>) -> Self {
        Self {
            name,
            key,
            salt,
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
                let mut data: Vec<String> = buf.split("\n").map(|s| s.to_string()).collect();

                let _salt_string = data.remove(0);
                let _name = data.remove(0);

                let salt = hex::decode(_salt_string.clone().to_owned()).unwrap();
                let key = generate_key(password.clone(), &salt);

                // just to confirm whether the vault name is correct
                let decrypted_name = decrypt(key, hex::decode(_name).unwrap())?;
                if decrypted_name != name {
                    println!("⛔ The vault is corrupt");
                }

                let mut vault = Self::new(name, key, salt);
                data.iter()
                    .map(|d| d.split_once('#').unwrap())
                    .for_each(|(k, v)| {
                        vault
                            .credentials
                            .insert(k.to_owned(), hex::decode(v).unwrap());
                    });

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
        let file_path = format!("{name}.vault");
        // check if file exists
        match OpenOptions::new()
            .read(true)
            .create_new(false)
            .open(file_path.clone())
        {
            Ok(_) => {
                println!("⛔ The vault with name '{name}' already Exists");
            }

            Err(_) => {
                // create a new vault
                let password = prompt_password("Enter password [hidden]: ").unwrap();
                let re_password = prompt_password("Confirm password [hidden]: ").unwrap();
                if password != re_password {
                    println!("⛔ Credentials do not match!!");
                    return;
                }
                // generate salt
                let salt = generate_salt(20);
                let key = generate_key(password.clone(), &salt);

                let mut vault = Vault::new(name, key, salt);
                vault.save(true);
            }
        };
    }

    pub(crate) fn set(&mut self, key: String, value: String) {
        let encoded = encrypt(self.key, value);
        self.credentials.insert(key, encoded);
        self.save(false);
    }

    pub(crate) fn get(&mut self, key: String) {
        match self.credentials.get(&key).as_ref() {
            Some(&value) => {
                let decrypted = decrypt(self.key, value.clone()).unwrap();
                message_box(format!("the credential for the given key is : {decrypted}"));
            }
            None => {
                println!("No credentials found for the given key")
            }
        };
    }

    fn save(&mut self, create_new: bool) {
        let file_path = format!("{}.vault", self.name);

        let mut data = Vec::new();
        data.push(hex::encode(self.salt.clone()));
        data.push(hex::encode(encrypt(self.key, self.name.clone())));
        self.credentials.iter().for_each(|(k, v)| {
            data.push(format!("{}#{}", k.as_str(), hex::encode(v)));
        });

        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create_new(create_new) // This will cause an error if the file already exists
            .open(file_path)
            .as_mut()
        {
            Ok(file) => {
                let _ = file.write(data.join("\n").as_bytes());
                println!(
                    "✅ The vault \"{}\" has been successfully created",
                    self.name
                );
            }
            Err(e) => {
                println!("⛔ Could not save the vault '{}'", self.name);
                println!("{e}");
            }
        }
    }
}
