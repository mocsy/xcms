#![warn(clippy::inline_fn_without_body)]
// #![feature(core_intrinsics)]
#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_diesel_actor as db;
use ecspg::schema;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[macro_use]
pub mod render;
mod menu;
pub mod modules;
mod utils;
