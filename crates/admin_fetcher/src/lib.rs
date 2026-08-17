#![doc = include_str!("../README.md")]

//! 
//! # Libs wrapper for admin_fetcher, act as OS plexer
//! 

use std::env;
use shared_libs::utils::*;

//: Os plexer
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::get;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::get;


pub fn asked_to_scan() {
    init_logger(env!("CARGO_PKG_NAME"));

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        log::error!("Missing info key.");
        std::process::exit(1);
    }

    let key = &args[1];

    if let Some(value) = get(key) {
        print!("{}", value); 
        std::process::exit(0);
    } else {
        log::error!("Key '{}' not found or access denied.", key);
        std::process::exit(1);
    }
}