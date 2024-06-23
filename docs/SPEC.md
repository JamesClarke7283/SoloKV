# Solo KV Design Document

## Introduction

SoloKV is a user-friendly key-value storage system that prioritizes simplicity and ease of use. Developed in Zig, it offers a minimalist API designed for developers who need a straightforward solution for storing and retrieving data. With its data stored in JSON format, SoloKV combines ease of use with the flexibility to handle various data types, making it a versatile choice for a wide range of applications.

## Main Advantages

- **Simple To Use**: SoloKV is built with the goal of being intuitive and straightforward, eliminating the complexity often associated with database systems. Its minimalistic API ensures that developers can easily integrate and work with the database without a steep learning curve.
- **Single File Storage**: All data within SoloKV is stored in a single file, simplifying data management, backup, and portability. This design choice makes it ideal for small to medium-sized projects, standalone applications, or any scenario where simplicity in data storage is desired.
- **Flexible Data Storage (WIP)**: Currently, SoloKV stores data in JSON format, striking a balance between readability and efficiency. Future updates aim to introduce support for non-JSON storage options, offering developers the flexibility to choose the optimal format for their specific needs while maintaining the simplicity at the core of SoloKV.

## Features

SoloKV simplifies data storage and retrieval with the following key features:

- `open`: Instantiates a new database connection, automatically detecting whether to inherit data from an existing file or to create a new database file.
- `close`: Closes the database connection.
- `keys`: Provides a list of all keys in the database, making it easy to overview the stored data.
- `get`: Obtains the value associated with a specific key, enabling precise data retrieval operations.
- `set`: Allows for the insertion or update of a key-value pair. Utilizing this function without specifying a value effectively deletes the key-value pair, offering a simple method for data management.

## Design Philosophy

SoloKV is designed with a focus on simplicity and efficiency, catering to developers who need a no-frills solution for key-value data storage. By maintaining the database in a single file and using JSON for data storage, SoloKV provides a balance between ease of use and flexibility, with an eye towards future enhancements that will offer even more storage options without compromising its core principles.

## Technical Overview

### Storage Engine

The Storage Engine is the core component of SoloKV, designed to support a schema-less design allowing for dynamic data types and structures. All data is stored within a single file, regardless of the chosen storage format.

### File Format and Serialization

SoloKV currently uses JSON for data storage, providing human-readable data storage. This format ensures maximum flexibility for users, allowing keys and values to be of various data types that can be serialized into JSON.

### Data Management

- **Writes and Updates**: Performed through the `set` command, directly modifying the data file to reflect changes.
- **Deletions**: Triggered by a `set` operation with no value, ensuring efficient use of storage space.
- **Reads**: Designed to be efficient and straightforward, utilizing the JSON structure to quickly locate data.

### Concurrency Control

Concurrent access is managed to ensure data integrity and consistency. The specific mechanism may be implemented as development progresses.

### Performance Optimization

SoloKV is designed with efficiency in mind, balancing performance with the goal of simplicity. Specific optimizations may be implemented as the project evolves.

## API Specification

SoloKV offers a simple yet powerful API designed for flexibility and efficiency. The main functions include:

- `open`: Initializes a database connection.
- `close`: Closes the database connection.
- `keys`: Lists all keys in the database.
- `get`: Retrieves a value associated with a key.
- `set`: Inserts or updates a key-value pair, or deletes a pair if no value is provided.

## Error Handling

SoloKV will implement robust error handling to ensure users are informed of issues related to file access, data integrity, or operation failures.

## Future Considerations

SoloKV is designed with future scalability in mind, though currently focusing on core functionalities. As the project evolves, additional features and optimizations may be considered, always keeping in mind the core principles of simplicity and ease of use.

SoloKV represents a straightforward approach to data storage, emphasizing user experience without sacrificing functionality. It's the perfect tool for developers seeking a lightweight, reliable, and easy-to-use database solution implemented in Zig.