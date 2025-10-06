#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_extern_crates)]

use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        log::error!("Panic occurred: {:?}", panic_info);
    }));

    if let Err(e) = std::panic::catch_unwind(|| maestro_lib::run()) {
        log::error!("Application crashed: {:?}", e);
    }
}
