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

### a. Stage 1 — File storage and serialization

record representation in memory and encoding it into byte sequence to be stored in disk.  
Tasks:

- Open and create a database file.
- Read and write byte ranges.
- Design a record format.
- Convert records to bytes and back.
- Detect malformed or truncated records.
- Reopen the database and recover previously stored data.
