# stonk

🐋 This project runs BDD tests and print the result of the different features and scenarios on the standard output. 🐟

## Prerequisites

- [rustc 1.50.0+](https://www.rust-lang.org/fr/)
- [Docker](https://www.docker.com/get-started)

## Quickstart

**Please note that before running the tests, you need to follow the following instructions to configure the project.**

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
  - `-- --debug` will print on the standard output the different data asked in the subject.

### Or run the cucumber tests with Docker

The `Dockerfile` offers an easy way to execute your tests.

- `docker-compose up --build`

## About the project

In this section I explain my decisions, the current state of the work and how
we can continue to improve it.

### Approach

This test project has been built with an end-to-end approach during an acceptance test phase. So, from a user perspective.
Doing so involve developers, product owners, business, QA, ... and it's where a Gherkin language will the different
stakeholders to discuss with a common language (ubiquitous language). It was one of the prerequisite of the test : using
a Gherkin BDD framework. Here we use an implementation of [Cucumber testing framework for Rust](https://github.com/bbqsrc/cucumber-rust).

### Environment variables

Developers can rely on the `.env` file (not tracked by git). CI pipelines need to create **secret (understand encrypted)** environment variables for obvious security issues.

- `API_BASE_URL` : The base URL to run tests against. This makes it easy to change the environment.
- `API_KEY` : The public API key generated from the website.
- `API_SECRET` : The private API key generated from the website.
- `TOPT_PWD` : When 2FA is activated for the API key. This variable is optional, but necessary in this test project for
  private endpoints. However, as defined in the exchange API doc, you can also disable the 2FA security.

### Code organization

I tried to build this test project following a specific team organization where test teams maintain their projects. We
are talking about end to end / acceptance tests and not integration tests. That's why this project is organized in two
parts :

- [`src`](src) : this folder contains the source code of the library with the domain layer (ubiquitous language, entities) and api helpers to call private and public methods. Doing so, we avoid to mix this logic in the tests and we can keep a separated code source for ease of maintenance. It can be seen as the testing tools to make the HTTP requests over the API. This folder could be mainly maintained by automation engineers / SDETS.

- [`tests`](tests) : this folder contains the features, using the Gherkin language, and the steps using the Cucumber framework.
  This folder could be mainly maintained by product owners (features), testers, automation engineers / SDETS.

Please note that I tried to bring some context in this test project. But it doesn't mean that every test projects shoud be organized in this way.

The domain layer can be a very important part. While I created a specific layer in this project, an alternative method would
be to use a specific crate with the different entities of the domain. This "domain crate" could therefore be shared between different teams in order to keep a very cohesive language and entities definitions accross
projects (test projects or application projects).

### CI

You can find the different workflow based on Github Actions in the [.github folder](.github/workflows). Here are the
different steps in general :

- Clippy / fmt : check the syntax, the errors and the best practices in Rust. Triggered on PR.
- Cargo audit : Check the vulnerabilities on push and schedule a daily check on HEAD. Triggered when changes in `Cargo.toml`
  or `Cargo.lock` are made (this is where dependencies are defined).
- Build and test : Try to build the library and run its unit tests. Try to build the BDD tests without running them.

- TODO : Please note that another workflow could be added to run BDD tests manually from Github Actions (or schedule / trigger them).

### Gherkin framework and reporting

For this project I used the open source BDD framework [cucumber-rust](https://github.com/bbqsrc/cucumber-rust).

> An implementation of the Cucumber testing framework for Rust. Fully native, no external test runners or dependencies.

Common features of BDD frameworks are available, however it's a relatively new project and some are missing. For example there is no way to get a Junit output which is used by a lot of reporting tools and CI/CD.

Due to the given time for this test project I can't implement this feature now but it could be the next step. Then it could be really easy to use [Allure Test Report](http://allure.qatools.ru/) or a test report to follow tests.

For me it's a very important feature since it allows people to get insights about the tests, their history. Another idea would be to use the API of Xray (a Jira plugin) and maintain tests cases with a test repository. Also the test organization could be better with a such tool : Test Execution, Test Plan, Test Set, Test.

### Async/Await

For this project stick as much as possible with async/await from Rust. It enables asynchronous test and reduces the time of test execution. Depending on the amount of tests and the time to market objectives, asynchronous tests can really improve test pipelines.

One of the reason I decided to implement my own API tools is because there is no official API crate, and I didn't find any crate with a HTTP client using asynchronous requests.

I chose [reqwest](https://github.com/seanmonstar/reqwest), and they explain the following thing in their documentation :

> You do not have to wrap the Client it in an Rc or Arc to reuse it, because it already uses an Arc internally.

## Tools and resources

Please find in this list the different tools et resources that help me to build this project :

- [cucumber-rust](https://github.com/bbqsrc/cucumber-rust)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [Json to Rust structures](https://transform.tools/json-to-rust-serde) : Turn json into Rust data structures deriving from serde
- [Coinnect repo](https://github.com/hugues31/coinnect/blob/master/src/kraken/api.rs) : I studied their API and crypto functions.

## Notes

### AssetPairs : a bug?

A difference exists between data returned by `AssetPairs?pair=xbtusd&info=info`
and data returned by `AssetPairs?pair=xbtusd&info=margin`.

Indeed, in one case we have `margin_stop` and in the other case we have `margin_level`.
