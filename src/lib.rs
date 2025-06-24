#![doc = include_str!("../README.md")]

pub mod browser;
#[cfg(feature = "cli")]
pub mod cli;
pub mod colors;
pub mod icon;
pub mod response;
pub mod topic;
pub mod user_agents;
