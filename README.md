# stonk

🐋 🐟

## Prerequisites

- rustc 1.50.0

## Quickstart

- `cp .env.example .env` and replace the values with yours.
- `cargo test`

## Docker

- `docker-compose up --build`

## Explanations

Some notes before a full redaction

### BDD

- Framework choice
- Vision
- Context : agile team -> PO, QA, Devs

### About async

- Async / await : reduce time
- Reqwest : You do not have to wrap the Client it in an Rc or Arc to reuse it, because it already uses an Arc internally.
