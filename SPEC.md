# Solo KV Design Document

## Introduction

Solo KV is a pioneering key-value storage system designed with the modern
application developer in mind. Developed in Rust and leveraging Protocol Buffers
(ProtoBuf) for serialization, Solo KV aims to redefine simplicity, flexibility,
and efficiency in data storage. By encapsulating the entire database within a
single file, Solo KV offers an unparalleled blend of performance and ease of use
for a wide array of applications, from lightweight web services to complex,
data-intensive applications.

At its core, Solo KV is about removing barriers. Whether you're a seasoned
developer or just starting out, Solo KV's intuitive interface and
straightforward setup process are designed to help you focus on what matters
most: building great applications.

## Philosophy

Solo KV is founded on three core principles, ordered by priority:

1. **Simplicity**: Our foremost goal is to ensure Solo KV remains easy to use.
   It's designed to integrate seamlessly into projects, offering an intuitive
   interface that simplifies data management without a steep learning curve.

2. **Flexibility**: Solo KV is built to serve a wide range of application needs,
   accommodating future expansions and different data formats. Its architecture
   is intentionally crafted to adapt to evolving project requirements and
   technological advancements.

3. **Efficiency**: While prioritizing simplicity and flexibility, Solo KV also
   focuses on efficiency. It balances performance and resource usage
   effectively, utilizing Rust's capabilities and ProtoBuf's serialization to
   deliver fast and compact data storage in a single-file system.

These principles guide every aspect of Solo KV's development, ensuring it
delivers a user-friendly, adaptable, and efficient data storage solution.

## Technical Overview

[WIP]

## Storage Engine

The Storage Engine is the core component of Solo KV, designed with the
principles of simplicity, flexibility, and efficiency in mind. It supports a
schema-less design, allowing for dynamic data types and structures, and is built
to store all data within a single file, regardless of the storage format chosen
by the user.

### File Format and Serialization

Solo KV supports two primary data storage formats: a binary format using serde's
binary serialization capabilities for efficiency, and JSON for human-readable
data storage. Upon initialization, users can specify their preferred
storage_format, with the binary format set as the default for its optimal
balance of performance and compactness.

The database is fundamentally schema-less, designed to accommodate a wide
variety of data types for both keys and values. This design choice ensures
maximum flexibility for users, allowing keys to be not just strings or byte
vectors (`Vec<u8>`), but any datatype that can be serialized and uniquely
identified, further extending to complex data types as needed. Similarly, values
can be any serializable data type, providing users the freedom to store diverse
and complex data structures.

This approach empowers Solo KV to handle a broad spectrum of application
requirements, ensuring that it can adapt to various data storage and retrieval
scenarios without imposing limitations on the structure or type of data being
stored.

### Data Management

Solo KV's data management strategy reflects a commitment to simplicity and
directness:

- **Writes and Updates**: Insertions and updates are performed through the `put`
  command, directly modifying the data file to reflect changes. If a key already
  exists, its value is overwritten, ensuring that only the most recent value is
  retained.

- **Deletions**: To maintain Solo KV's commitment to optimizing storage space,
  deletions—triggered by a `put` operation with an empty value. This approach,
  while potentially slower for large files, ensures efficient use of storage
  space by avoiding the retention of obsolete data.

- **Reads**: Read operations are designed to be efficient and straightforward,
  utilizing an in-memory index to quickly locate data in the file without
  scanning the entire storage medium for each request.

### Concurrency Control

Concurrent access is managed using a Read-Write lock mechanism, ensuring that
read operations can occur simultaneously without blocking, while write
operations obtain exclusive access to prevent data corruption and ensure
consistency. This model prioritizes write operations, requiring read operations
to wait, thereby ensuring that reads always reflect the most up-to-date state of
the database following any writes.

### Performance Optimization

Solo KV is designed for efficiency, employing strategies such as in-memory
caching of frequently accessed data to reduce disk I/O and write buffering to
minimize the impact of disk write latency. These optimizations are balanced
against the goal of simplicity, ensuring that Solo KV remains easy to use and
integrate into various projects without requiring extensive configuration.

### No Transaction Journaling

In keeping with Solo KV's philosophy of simplicity and efficiency, the storage
engine does not implement transaction journaling or logging. This decision
streamlines the architecture and operation of Solo KV, focusing on direct and
straightforward data interactions without the overhead of managing transaction
logs.

## API Specification

Solo KV offers a simple yet powerful API designed for flexibility and
efficiency. This section details the function signatures, their usage, and
expected outcomes, aligning with Solo KV's commitment to simplicity,
flexibility, and efficiency.

### Initializing a Database Connection

```rust
fn new(path: String, format: Option<StorageFormat>) -> Result<Database, SoloError>
```

- **`path`**: The file path (either relative or absolute) where the database is
  stored.
- **`format`** (Optional): Specifies the storage format (`Binary` or `JSON`).
  Defaults to `Binary` if not provided.
- **Returns**: A result wrapping the database connection or an error.

### Listing All Keys

```rust
fn keys() -> Vec<DynamicType>
```

- **Returns**: A vector containing all keys in the database in their stored data
  types.

### Retrieving a Value

```rust
fn get(key: DynamicType) -> Result<DynamicType, SoloError>
```

- **`key`**: The key for which to retrieve the value, which can be of any data
  type.
- **Returns**: A result wrapping the value associated with the key or an error.

### Inserting or Updating a Value

```rust
fn put(key: DynamicType, value: Option<DynamicType>) -> Result<(), SoloError>
```

- **`key`**: The key to insert or update, which can be of any data type.
- **`value`** (Optional): The value to associate with the key. If `None`, the
  key is deleted.
- **Returns**: A result indicating success (`Ok`) or an error (`Err`).

### Checking Existence of a Key

```rust
fn exists(key: DynamicType) -> bool
```

- **`key`**: The key to check for existence, which can be of any data type.
- **Returns**: `true` if the key exists, `false` otherwise.

### Error Handling

Solo KV uses the `thiserror` crate for custom error handling, ensuring that all
functions return a `Result` type. This approach allows operations to either
succeed, returning `Ok` with any relevant data, or fail, returning an `Err` with
an error description.

### Concurrency

Concurrency is managed internally with a Read-Write lock mechanism, abstracting
away complexity from the user while maintaining thread safety. This ensures that
reads can happen concurrently, but writes obtain exclusive access to prevent
data corruption.

### Best Practices and Performance

While specific performance optimizations are ongoing, Solo KV is designed to
efficiently manage data within a single file storage system. Users are
encouraged to select the appropriate storage format based on their needs for
readability (JSON) or efficiency (Binary).

### Future Considerations and Versioning

Solo KV is designed with future scalability in mind, though currently focusing
on core functionalities. As the project evolves, additional features may be
considered. Solo KV does not support database versioning or legacy formats,
emphasizing simplicity and forward compatibility.

Based on your specifications and preferences, here is the "## Data Handling"
section tailored for Solo KV, reflecting the changes and decisions you've made:

---

## Data Handling

Solo KV is engineered to provide a straightforward and efficient mechanism for
storing, updating, and retrieving key-value pairs, strictly adhering to the
principles of simplicity and flexibility. This section outlines the strategies
and methodologies Solo KV employs to manage data within its single-file storage
system.

### Storage Model

Solo KV utilizes a single-file storage model, with data serialized using either
Serde for binary format or Serde_json for JSON format, depending on user
preference. The database operates under a schema-less design, accommodating any
data type for both keys and values. This approach ensures flexibility in storing
varied data structures, from simple text to complex nested objects.

- **Initialization**: Upon creating a new database connection with the `new`
  function, users must specify a `path` and may optionally define the
  `storage_format` (defaulting to Serde's binary format for efficiency). Solo KV
  verifies the existence and format of the specified database file, ensuring
  compatibility or initializing a new file as needed.

- **Data Representation**: Within the database file, each entry is stored in a
  key-value pair format, `{"key": "value"}`, where both the key and value can be
  of any serializable data type. This design choice supports a wide range of
  applications and data storage requirements.

### Data Operations

- **Key Retrieval**: The `keys` function returns a vector of all keys present in
  the database, allowing users to easily overview the stored data. The keys are
  returned in their original data types, ensuring type consistency throughout
  the database operations.

- **Data Retrieval**: The `get` function requires an exact key match to retrieve
  the corresponding value. This operation emphasizes the importance of precise
  key management within Solo KV's usage.

- **Data Insertion and Update**: The `put` function inserts a new key-value pair
  or updates an existing pair. If the specified key already exists, its value is
  overwritten, demonstrating Solo KV's straightforward approach to data
  management. Specifying a key with no value effectively deletes the pair from
  the database, aligning with the simplicity of Solo KV's design.

- **Existence Check**: The `exists` function provides a simple mechanism to
  check for the presence of a key within the database, returning a boolean
  value.

### Concurrency Management

Solo KV employs a Read-Write lock mechanism to manage concurrent access,
ensuring data integrity without complicating the user experience. Write
operations are prioritized, with read operations proceeding once writes are
completed. This strategy guarantees that read operations always access the most
up-to-date data. Solo KV abstracts away the complexities of concurrency control,
providing a seamless experience for users.

### Data Integrity and Space Management

While Solo KV does not support transaction journaling or rollback capabilities,
it is designed to maintain high data integrity through careful operation
handling. Users are encouraged to perform regular backups to safeguard their
data. The deletion of key-value pairs is handled with care to ensure the
database file remains compact and efficient, with Solo KV implementing
strategies to fully remove entries from the storage file upon deletion requests.

### Performance Considerations

Solo KV is optimized for performance within its single-file storage model. The
choice between binary and JSON formats allows users to balance between
efficiency and readability, catering to various use case requirements. Further
optimizations, including in-memory caching and batch processing, are considered
for future enhancements to improve Solo KV's performance and efficiency.
