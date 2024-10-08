//! Core traits for the [`arithmetic-coding-adder-dep`](https://github.com/danieleades/arithmetic-coding) crate

#![deny(missing_docs, clippy::all, missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![feature(associated_type_defaults)]

mod bitstore;
pub use bitstore::BitStore;

mod model;
pub use model::{fixed_length, max_length, one_shot, Model};
