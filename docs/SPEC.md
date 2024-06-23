# Solo KV Design Document

## Introduction

SoloKV is a user-friendly key-value storage system that prioritizes simplicity and ease of use. Developed in Zig, it offers a minimalist API designed for developers who need a straightforward solution for storing and retrieving data. With its data stored in BSON format and utilizing in-memory storage with an Append-Only File (AOF), SoloKV combines efficiency with the flexibility to handle various data types, making it a versatile choice for a wide range of applications.

## Main Advantages

- **Simple To Use**: SoloKV is built with the goal of being intuitive and straightforward, eliminating the complexity often associated with database systems. Its minimalistic API ensures that developers can easily integrate and work with the database without a steep learning curve.
- **Single File Storage**: All data within SoloKV is stored in a single file, simplifying data management, backup, and portability. This design choice makes it ideal for small to medium-sized projects, standalone applications, or any scenario where simplicity in data storage is desired.
- **Efficient Data Storage**: SoloKV uses BSON (Binary JSON) for data storage, striking a balance between efficiency and readability. This format allows for faster parsing and serialization compared to JSON while maintaining flexibility in data types.
- **In-Memory Performance**: By keeping data in memory and using an Append-Only File for durability, SoloKV provides extremely fast read and write operations.

## Features

SoloKV simplifies data storage and retrieval with the following key features:

- `open`: Instantiates a new database connection, loading data from an existing file or creating a new one.
- `close`: Closes the database connection.
- `keys`: Provides a list of all keys in the database, making it easy to overview the stored data.
- `get`: Obtains the value associated with a specific key, enabling precise data retrieval operations. Returns null if the key doesn't exist.
- `set`: Allows for the insertion, update, or deletion of a key-value pair. Setting a value to null effectively deletes the key-value pair.
- `compact`: Triggers a manual compaction of the Append-Only File.
- `sync`: Writes the current in-memory state back to the file opened with "open".

## Design Philosophy

SoloKV is designed with a focus on simplicity, efficiency, and in-memory performance, catering to developers who need a no-frills solution for key-value data storage. By maintaining the database in a single file using BSON for storage and leveraging in-memory operations with an Append-Only File, SoloKV provides a balance between ease of use, flexibility, and performance.

## Technical Overview

### Storage Engine

The Storage Engine is the core component of SoloKV, designed to support a schema-less design allowing for dynamic data types and structures. The primary storage is in-memory, with all operations logged to an Append-Only File (AOF) for durability and crash recovery.

### File Format and Serialization

SoloKV uses BSON (Binary JSON) for both in-memory representation and AOF storage. This format ensures efficient serialization and deserialization while maintaining flexibility for various data types.

### Data Management

- **Writes and Updates**: Performed in-memory and logged to the AOF, ensuring fast operations and durability.
- **Deletions**: Triggered by a `set` operation with a null value, logged to the AOF, and removed from in-memory storage.
- **Reads**: Extremely fast, as data is primarily accessed from memory.

### Concurrency Control

SoloKV implements Optimistic Concurrency Control (OCC) to manage concurrent access, ensuring data integrity and consistency while providing high performance in low-contention scenarios.

### Performance Optimization

SoloKV is optimized for in-memory operations, with periodic compaction of the AOF to manage file size and improve startup times.

## API Specification

SoloKV offers a simple yet powerful API designed for flexibility and efficiency. The main functions include:

- `open(path: []const u8) !SoloKV`: Initializes a database connection.
- `close() void`: Closes the database connection.
- `keys() ![][]const u8`: Lists all keys in the database.
- `get(key: []const u8) !?[]const u8`: Retrieves a value associated with a key. Returns null if the key doesn't exist.
- `set(key: []const u8, value: ?[]const u8) !void`: Inserts or updates a key-value pair. If value is null, it deletes the key-value pair.
- `compact() !void`: Manually triggers compaction of the AOF.
- `sync() !void`: Writes the current in-memory state to the file opened with "open".

Note: Keys are restricted to string datatypes only.

## Error Handling

SoloKV implements robust error handling to ensure users are informed of issues related to file access, data integrity, or operation failures.

## Future Considerations

SoloKV is designed with future scalability in mind, though currently focusing on core functionalities. As the project evolves, additional features and optimizations may be considered, always keeping in mind the core principles of simplicity, ease of use, and in-memory performance.

SoloKV represents a straightforward approach to data storage, emphasizing user experience and in-memory performance without sacrificing functionality. It's the perfect tool for developers seeking a lightweight, reliable, and easy-to-use database solution implemented in Zig.