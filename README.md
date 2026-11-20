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

a. ### Public api

- open database
- execute query
- fetch rows

b. ### SQL frontend

Tokenizer → Parser → AST → Validation

c. ### Query execution engine

- Scans
- Filters
- Inserts
- Updates
- Deletes

d. ### Storage engine

- Tables
- B-tree
- Record
- serialization

f. ### Persistence layer

- PagesBuffer
- pool
- File I/O
- WAL
