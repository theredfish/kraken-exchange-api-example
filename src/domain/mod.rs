//! This module contains the data structures representing the domain under test.
//!
//! Generally this domain layer exists in applications following the
//! "Domain Driven Design" methodology. This domain is part of the ubiquitous
//! language shared by developers and domain experts.
//!
//! BDD tests can take advantage of this domain layer and share the same data
//! structures. In our case we define here the different structures that we need
//! in our tests and their serde::Deserialize implementation.

pub mod asset_pair;
pub mod time;
