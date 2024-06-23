# Solo KV

SoloKV is a user-friendly key-value storage system that prioritizes simplicity
and ease of use. Developed in Rust, it offers a minimalist API designed for
developers who need a straightforward solution for storing and retrieving data.
With its data stored in JSON format, SoloKV combines ease of use with the
flexibility to handle various data types, making it a versatile choice for a
wide range of applications.

## Main Advantages

- **Simple To Use**: SoloKV is built with the goal of being intuitive and
  straightforward, eliminating the complexity often associated with database
  systems. Its minimalistic API ensures that developers can easily integrate and
  work with the database without a steep learning curve.
- **Single File Storage**: All data within SoloKV is stored in a single file,
  simplifying data management, backup, and portability. This design choice makes
  it ideal for small to medium-sized projects, standalone applications, or any
  scenario where simplicity in data storage is desired.
- **Flexible Data Storage (WIP)**: Currently, SoloKV stores data in JSON format,
  striking a balance between readability and efficiency. Future updates aim to
  introduce support for non-JSON storage options, offering developers the
  flexibility to choose the optimal format for their specific needs while
  maintaining the simplicity at the core of SoloKV.

## Features

SoloKV simplifies data storage and retrieval with the following key features:

- `open`: Instantiates a new database connection, automatically detecting whether
  to inherit data from an existing file or to create a new database file.
- `close`: Closes the database connection.
- `keys`: Provides a list of all keys in the database, making it easy to
  overview the stored data.
- `get`: Obtains the value associated with a specific key, enabling precise data
  retrieval operations.
- `set`: Allows for the insertion or update of a key-value pair. Utilizing this
  function without specifying a value effectively deletes the key-value pair,
  offering a simple method for data management.

## Design Philosophy

SoloKV is designed with a focus on simplicity and efficiency, catering to
developers who need a no-frills solution for key-value data storage. By
maintaining the database in a single file and using JSON for data storage,
SoloKV provides a balance between ease of use and flexibility, with an eye
towards future enhancements that will offer even more storage options without
compromising its core principles.

SoloKV represents a straightforward approach to data storage, emphasizing user
experience without sacrificing functionality. It's the perfect tool for
developers seeking a lightweight, reliable, and easy-to-use database solution.
