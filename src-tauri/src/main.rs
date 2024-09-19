// Prevents additional console window on Windows in release, DO NOT REMOVE!!

// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(warnings)]

use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        log::error!("Panic occurred: {:?}", panic_info);
    }));

    if let Err(e) = std::panic::catch_unwind(|| mix_monkey_lib::run()) {
        log::error!("Application crashed: {:?}", e);
    }
}
