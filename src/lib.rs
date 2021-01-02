#![allow(unknown_lints)]
#![recursion_limit = "1024"]
#![cfg_attr(feature = "nightly", feature(start))]
#![cfg_attr(feature = "nightly", feature(ord_max_min))]

#[macro_use]
extern crate scopeguard;
#[macro_use]
extern crate log;
// TODO
// use log::{info, debug, trace, warn, error};

// use color_eyre::eyre::
#[macro_use]
extern crate lazy_static;

pub mod capabilities;
pub mod cgroups;
pub mod consts;
pub mod errors;
pub mod logger;
pub mod mounts;
pub mod nix_ext;
pub mod seccomp;
pub mod selinux;
pub mod signals;
pub mod sync;
