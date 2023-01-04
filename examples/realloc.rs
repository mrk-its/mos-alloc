#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String, vec::Vec};

use ufmt_stdio::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    loop {}
}

static DATA: &str = "0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15";

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // reserve small amount of memory for heap
    // (32 bytes for vec data + some overhead)
    // it will fail if true realloc is not available

    mos_alloc::set_limit(50);

    let initial_bytes_free = mos_alloc::bytes_free();
    println!(
        "heap bytes free: {} / {}",
        initial_bytes_free,
        mos_alloc::get_limit(),
    );

    {
        let vec = DATA
            .split(",")
            .map(|i| i.parse::<i16>().unwrap())
            .collect::<Vec<_>>();
        println!("allocated, free: {}", mos_alloc::bytes_free());
    }
    assert!(mos_alloc::bytes_free() == initial_bytes_free);
    println!("deallocated, free: {}", mos_alloc::bytes_free());
    0
}