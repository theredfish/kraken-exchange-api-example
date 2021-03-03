# stonk

🐋 🐟

## Prerequisites

- [rustc 1.50.0+](https://www.rust-lang.org/fr/)
- [Docker](https://www.docker.com/get-started)

## Quickstart

This where you want to start before doing anything. You have some actions to do
before running the tests.

This project will generate a standard output with the result of the different
features and scenarios following the Gherkin language.

### Setup your API key and TOTP information

- `cp .env.example .env`
- Create an API key on the website.
  - Report your private and public keys in your `.env` : please note that it's the only moment where you will see them
  - Tick the `Query Open Orders & Trades` permission
  - If you want setup a key expiration date
  - Configure the 2FA settings for the API key and change the method to password
  - Report the value of your password in your `.env` for the key `TOTP_PWD`
  - For better results create new orders / trades on this account

### Run the cucumber tests with Cargo

- `cargo test --test bdd -- --debug`
  - `cargo test --test bdd` run the cucumber tests
  - `debug` will print on the standard output the different data asked in the subject.

### Or run the cucumber tests with Docker

The `Dockerfile` offers an easy way to execute your tests

- `docker-compose up --build`

## Explanations

In this section I explain my decisions, the current state of the work and how
we can continue to improve it.

### Context

### Gherkin framework

- Framework choice
- Vision
- Context : agile team -> PO, QA, Devs

### Async/Await

- Async / await : reduce time
- Reqwest : You do not have to wrap the Client it in an Rc or Arc to reuse it, because it already uses an Arc internally.

## Tools and resources

Please find in this list the different tools et resources that help me :

- [Json to Rust structures](https://transform.tools/json-to-rust-serde) : Turn json into Rust data structures deriving from serde
- [Coinnect repo](https://github.com/hugues31/coinnect/blob/master/src/kraken/api.rs) : I studied their API and crypto functions.

## Notes

### AssetPairs : a bug?

A difference exists between data returned by https://api.kraken.com/0/public/AssetPairs?pair=xbtusd&info=info
and data returned by https://api.kraken.com/0/public/AssetPairs?pair=xbtusd&info=margin.

Indeed, in one case we have `margin_stop` and in the other case we have `margin_level`.
