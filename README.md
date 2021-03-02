# stonk

🐋 🐟

## Prerequisites

- rustc 1.50.0

## Quickstart

- `cp .env.example .env` and replace the values with yours.
- `cargo test --test bdd -- --debug`

`debug` will print reports asked in the subject.

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

## Useful tools

- [Turn json into Rust data structures deriving from serde](https://transform.tools/json-to-rust-serde)

## Notes

### AssetPairs : a bug?

A difference exists between data returned by https://api.kraken.com/0/public/AssetPairs?pair=xbtusd&info=info
and data returned by https://api.kraken.com/0/public/AssetPairs?pair=xbtusd&info=margin.

Indeed, in one case we have `margin_stop` and in the other case we have `margin_level`.
