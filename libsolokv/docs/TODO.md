# Todo List for Solo KV

## Must have

- [ ] Add 100% test coverage
- [ ] Add criterion benchmarking
- [ ] Add github actions for linting,testing,benching.
- [ ] Add command-line utility so you can operate on the database from the command-line.
- [ ] Improve performance of database by using LRU caching and also use Indexing, read only relevant parts, not whole file.

## Should have

- [ ] Use URI for new database connections, so as to later support networking.
- [ ] Add authentication and networking so the database can be accessed over a network
- [ ] Data Versioning: Introduce data versioning to keep track of changes over time, allowing rollback to previous states.

## Could have

- [ ] REST API Interface: Develop a RESTful API layer for accessing and manipulating the database over HTTP, enabling integration with web services and applications.

- [ ] Zero Knowledge Encryption: Add support for encrypting data stored on disk and in transit to enhance security for sensitive information.

## Won't have

- [ ] Query Language Development: Reason being that this is a KV database not a document database, or other more advanced ones.

- [ ] Data Compression: Implement data compression options to minimize disk space usage, especially for large datasets(check zstd compression). Wont implement because it has performance overhead, and we keep the database pretty minimal anyway.