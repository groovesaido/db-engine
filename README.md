# SINGLE FILE DATABASE ENGINE

**_AUTHORS_**
| NAME | USERNAME | EMAIL | PHONE |
| ---| --- | --- | --- |
|Stephen Muiga | groovesaido | stevekiragu254@gmail.com | 0793682713 |

SQL text → Tokenizer → Parser (AST) → Planner → Executor

↓
B+Tree (tables & indexes)

↓
Pager + Page Cache

↓
Single database file (+ journal/WAL)

## 1 PROJECT SCOPE

db-engine is a relational,serverless,embedded database similar to sqlite.

| PROPERTY        | Target               |
| --------------- | -------------------- | --------------------------- |
| language        | Rust                 |
| Deployment      | Embedded             |
| Storage         |                      | One database file           |
| Data model      |                      | Tables, rows and columns    |
| Primary index   |                      | B-tree                      |
| Query interface |                      | A subset of SQL             |
| Transactions    |                      | Atomic commits and rollback |
| Concurrency     | Initially one writer |
| External server |                      | None                        |

## 2 DATABASE ARCHITECTURE

### a. Public api

- open database
- execute query
- fetch rows

### b. SQL frontend

Tokenizer → Parser → AST → Validation

### c. Query execution engine

- Scans
- Filters
- Inserts
- Updates
- Deletes

### d. Storage engine

- Tables
- B-tree
- Record
- serialization

### e. Persistence layer

- PagesBuffer
- pool
- File I/O
- WAL

## 3. project Implementation

### Stage 1 — File storage and serialization

record representation in memory and encoding it into byte sequence to be stored in disk.

Tasks:

- Open and create a database file.
- Read and write byte ranges.
- Design a record format.
- Convert records to bytes and back.
- Detect malformed or truncated records.
- Reopen the database and recover previously stored data.

**Milestone: Insert a record, exit the process, restart it, and retrieve the same record.**

### Stage 2 — Fixed-size pages and a page manager

We will implement a page of a fixed size to be decided between 4KB or 8KB.

The page manager will need to:

- Translate a page number into a file offset.
- Read and write complete pages.
- Track allocated pages.
- Allocate new pages.
- Maintain metadata describing the database.

The offset of page _p_ is:

```text
offset(_p_) = _p_ &times; page size
```

implement page header, usable space, record layout, and how to detect corrupted pages.

**Milestone: The database can create, read, modify, and allocate pages independently of the higher-level data model.**

### Stage 3 — B-tree storage

A B-tree data structure to organize keys (indexing) and reduce search time complexities.

implementation:

- Search for a key.
- Insert into a leaf.
- Split an overflowing leaf.
- Propagate splits to parent nodes.
- Split the root when necessary.
- Traverse records in sorted order.
- Delete records and handle underflow.

A B+ tree, with records or record references in its leaves and keys in internal nodes, is a strong choice for range scans.

**Milestone: Insert thousands of keys in random order and retrieve every key correctly without scanning every record.**
