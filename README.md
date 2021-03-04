# stonk

🐋 This project runs BDD tests and print the result of the different features and scenarios on the standard output. 🐟

## Prerequisites

- [rustc 1.50.0+](https://www.rust-lang.org/fr/)
- [Docker](https://www.docker.com/get-started)
- Exchange account + pub/pvt API keys + 2FA password (totp)

## Quickstart

**Please note that before running the tests, you need to follow the following instructions to configure the project.**

### Setup your API key and TOTP information

- `cp .env.example .env`
- Create an API key on the website.
  - Report your private and public keys in your `.env` : please note that it's the only moment where you will see them
  - Tick the `Query Open Orders & Trades` permission
  - If you want setup a key expiration date
  - Configure the 2FA settings for the API key and change the method to password
  - In order to avoid nonce errors for very quick successions in order, set your nonce windows to 5000
  - Report the value of your password in your `.env` for the key `TOTP_PWD`
  - For better results create new orders / trades on this account

### Run the cucumber tests with Cargo

- `cargo test --test bdd -- --debug`
  - `cargo test --test bdd` run the cucumber tests
  - `-- --debug` will print on the standard output the reporting.

### Or run the cucumber tests with Docker

The `Dockerfile` offers an easy way to execute your tests.

With Docker :

- `docker build --no-cache -t bdd .`
- `docker run --env-file .env bdd`

/!\ Be careful Docker doesn't handle quotation marks in `.env` file, [they are part of the VAL](https://docs.docker.com/compose/env-file/) :

> There is no special handling of quotation marks. This means that they are part of the VAL.

With docker-compose :

- `docker-compose up`

### Endpoints under test

- public
  - Time : 1 api call
  - AssetPairs : 3 different api calls with data driven. A bug may have been encountered, see [Other Notes](#other-notes) for more information
- private
  - OpenOrder : 1 api call

This is the result you should expect, one test case fails. It's a good way to demonstrate how BDD tests can help to find bugs :

```
[Summary]
3 features
5 scenarios (1 failed, 4 passed)
40 steps (1 failed, 19 passed)
```

## About the project

In this section I explain my decisions, the current state of the work and how
we can continue to improve it.

### Technical documentation

[Available here](http://www.juliandidier.fr/stonk)

### Approach

This test project has been built with an end-to-end approach during an acceptance test phase. So, from a user perspective. This kind of approach generally involve developers, product owners, business, QA, ... it's where Gherkin help to bring a common language (ubiquitous language). Here we use an implementation of [Cucumber testing framework for Rust](https://github.com/bbqsrc/cucumber-rust).

For this project I took a little bit more technical vocabulary where API endpoints are written in the Features. It allowed me to create generic and reusable Steps. This choice depends generally of the team, and should be discuss.

### Environment variables to configure API access

Developers can rely on the `.env` file (not tracked by git). Continuous integration pipelines need to configure **secret (understand encrypted)** environment variables for obvious security issues.

- `API_BASE_URL` : The base URL to run tests against. This makes it easy to change the environment.
- `API_KEY` : The public API key generated from the website.
- `API_SECRET` : The private API key generated from the website.
- `TOPT_PWD` : When 2FA is activated for the API key. This variable is optional, but necessary in this test project for
  private endpoints. However, as defined in the exchange API doc, you can also disable the 2FA security.

### Code organization

I tried to build this test project following a specific organization where test teams keep their projects separated from others.

Please note that I implemented end-to-end / acceptance tests and not integration tests, but took advantage of the way integration testing is handled by Cargo to run BDD tests. Please read the next section [A better code organization](#a-better-code-organization) for more details.

I also implemented a library to help me with API calls, cryptography, and to define the domain layer. That's why this project is organized in two
parts :

- [`src`](src) : this folder contains the source code of the library with the domain layer (ubiquitous language, entities) and the api helpers to call the private and public methods. By doing this, we avoid mixing this logic in the tests and we can keep separate source code for easier maintenance. We can consider this library as a set of test tools to facilitate HTTP requests via the API. This library could mainly be maintained by automation engineers / SDETS.

- [`tests`](tests) : this folder contains the "Features", using the Gherkin language, and the "Steps" using the Cucumber framework.
  This folder could mainly be maintained by product owners/testers (features), and automation engineers / SDETS (steps).

Please note that I tried to provide an organizational context similar to that which can be found in a company. But it doesn't mean that every test projects shoud be organized in this way.

I have some experience with projects following the "Domain Driven Design" methodology. When I work on a project, I choose certain concepts of DDD, while keeping a good balance between the complexity of the code and the complexity of the business domain.

### A better code organization

To improve code distribution, compilation time and maintenance, a better organization of the code would have been to separate the library and publish it as a separate crate.

By doing this the test project can simply import the library and use it to make http calls to private and public endpoints without additional compilation time while building the tests.

**I took a shortcut here, and took advantage of the way integration testing is handled by Cargo :**

1. the library (and binary if available) are compiled
2. the integration tests are compiled and executed (they `use` the library)

This saves me from creating separate crates for the library and the tests. It's easier to maintain, easier to deploy/run (no versioning), but with the added cost of build time.

Another solution would be to use a workspace with two separate projects for the library and the tests for better code maintenance. The library could therefore be versioned and imported into serveral test projects (from the same workspace or not, depending on the organization on the test projects).

### CI

You can find the different workflow based on Github Actions in the [.github folder](.github/workflows). Here are the
different steps in general :

- Clippy / fmt : check the syntax, the errors and the best practices in Rust. Triggered on PR.
- Cargo audit : Check the vulnerabilities on push and schedule a daily check on HEAD. Triggered when changes in `Cargo.toml`
  or `Cargo.lock` are made (this is where dependencies are defined).
- Build and test : Try to build the library and run its unit tests. Try to build the BDD tests without running them.
- A manual workflow to execute tests from Github Actions. [See here](https://github.com/theredfish/stonk/actions/workflows/bdd-manual-dispatch.yml)

### Gherkin framework and reporting

For this project I used the open source BDD framework [cucumber-rust](https://github.com/bbqsrc/cucumber-rust).

> An implementation of the Cucumber testing framework for Rust. Fully native, no external test runners or dependencies.

Common features of BDD frameworks are available, but this is a relatively new project and some features are missing. For example, there is no Junit output, which is used by many reporting tools and CI/CD pipelines ([see issue](https://github.com/bbqsrc/cucumber-rust/issues/50)). Reporting is currently performed on standard output. The next step could be to listen to the different events of this Cucumber framework and create a report in the Junit format, and map the data into a report like [Allure Test Report](http://allure.qatools.ru/). Another idea would be to use the Xray's API (a Jira plugin) and maintain tests in a test repository. The test organization could be better with such a tool : Test Execution, Test Plan, Test Set, Test.

### Async/Await

For this project I stuck with Rust's async/await as much as possible. It enables asynchronous testing and reduces test execution time. Depending on the number of tests and time to market, asynchronous testing can really improve test pipelines.

One of the reasons I decided to implement my own API tools is because there is no official API crate, and I couldn't find any crate with a HTTP client using asynchronous requests. Plus, I learned a lot about how you handle the crypto part and how your API works!

I chose [reqwest](https://github.com/seanmonstar/reqwest) as my asynchronous http client. They explain the following thing in their documentation concerning the concurrent access :

> You do not have to wrap the Client it in an Rc or Arc to reuse it, because it already uses an Arc internally.

## Tools and resources

Please find in this list the different tools et resources that help me to build this project :

- [cucumber-rust](https://github.com/bbqsrc/cucumber-rust)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [Json to Rust structures](https://transform.tools/json-to-rust-serde) : Turn json into Rust data structures deriving from serde
- [Coinnect repo](https://github.com/hugues31/coinnect/blob/master/src/kraken/api.rs) : I studied their API and crypto functions.

## Other Notes

### AssetPairs : a bug?

During my personal tests I found that a difference exists between data returned from `AssetPairs?pair=xbtusd&info=info`
and data returned from `AssetPairs?pair=xbtusd&info=margin`.

Indeed, in one case we have `margin_stop` and in the other case we have `margin_level` (not documented on your website).

See scenario "Retrieve margin information from an asset pair"
