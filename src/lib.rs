#![doc = include_str!("../README.md")]

mod pos;
mod scanner;
pub use pos::WithPos;
pub use scanner::{MatchType, Scanny};
