//! Utilities for tests only.

use env_logger::{Builder, Target};
use log::LevelFilter;

mod data_helper;
mod type_id;

pub(crate) use ckb_bitcoin_spv_prover::utilities::decode_from_bin_file;
pub(crate) use data_helper::{find_bin_file, find_bin_files};
pub(crate) use type_id::calculate_type_id;

pub(crate) fn setup() {
    let _ = Builder::new()
        .filter_module("tests", LevelFilter::Trace)
        .filter_module("ckb_bitcoin_spv", LevelFilter::Trace)
        .target(Target::Stdout)
        .is_test(true)
        .try_init();
    println!();
}

pub(crate) fn prev_client_id(current: u8, count: u8) -> u8 {
    if current == 0 {
        count - 1
    } else {
        current - 1
    }
}

pub(crate) fn next_client_id(current: u8, count: u8) -> u8 {
    if current + 1 < count {
        current + 1
    } else {
        0
    }
}
