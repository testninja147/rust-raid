# Vault

`vault` is a basic password vault that stores encrypted keys and values.
This library demonstrates the basics of storing the encrypted data.

The encryption process includes the following steps to ensure the security of data:

1. Generate unique`Salt` and create a hashed key using `Argon2` Hasher
2. Generate unique`Nonce`
3. Create an encrypted `cypher` with the hashed `key` and `nonce` so that the
   `data` can not be reverse engineered even if same password is used to encrypt
   same data multiple times.
4. Encode the `cypher` as a hex string to store in a file. (demo only)

to run the project, you can run the following command:

```shell
cargo run --bin vault
```

to build and use, you can run the following command:

```shell
cargo build --release --bin vault
```
## Example Shell output

### Creating a vault

+------------------------------------------------------------------------------+
|                             open | create | exit                             |
+------------------------------------------------------------------------------+
🔒: create
Vault Name: default
Enter password [hidden]:

Confirm password [hidden]:

()
✅ The vault "default" has been successfully created

### Opening a vault
```
+------------------------------------------------------------------------------+
|                             open | create | exit                             |
+------------------------------------------------------------------------------+
🔒: open
Enter Vault Name: default
Enter Password [hidden]:

()
✅ The vault is unlocked
```

### listing credentials (when vault is unlocked)
```
+------------------------------------------------------------------------------+
|            list | get <key> | push <key> <val> | pop <key> | lock            |
+------------------------------------------------------------------------------+
[ default ] 🔓: list
| test1@example.com              | *****                                       |
| test@example.com               | *****                                       |
