# SimpleKV

SimpleKV is a straightforward key-value storage system, emphasising simplicity
over complex features. Developed in Rust, SimpleKV focuses on providing a
minimalist API for storing and retrieving data, with all data kept in JSON
format for ease of use and flexibility.

## Features

SimpleKV is designed to be intuitive and easy to use, offering the following
functionalities:

- `new`: Creates a new database connection. If the specified database file
  exists, it inherits the data from that file. Otherwise, it creates a new file
  for the database.
- `keys`: Lists all keys currently in the database.
- `values`: Retrieves all values in the database without their corresponding
  keys.
- `get`: Fetches the value associated with a specific key.
- `put`: Inserts or updates a key/value pair in the database. If no value is
  provided for a key, the key-value pair is deleted.
- `exists`: Checks if a key exists in the database and has an associated value.

SimpleKV's design philosophy prioritises ease of use, making it an ideal choice
for projects requiring a lightweight and straightforward data storage solution.
