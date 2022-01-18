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

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let initial_bytes_free = mos_alloc::bytes_free();
    println!(
        "heap bytes free: {} / {}",
        initial_bytes_free,
        mos_alloc::get_limit(),
    );

    {
        let text = String::from("foo");
        println!(
            "allocated string {}, free: {}",
            &text[..],
            mos_alloc::bytes_free()
        );
        {
            let data = (0..50u16).collect::<Vec<_>>();
            println!(
                "allocated vec: {:?}, free: {}",
                &data[..],
                mos_alloc::bytes_free()
            );
        }
    }
    assert!(mos_alloc::bytes_free() == initial_bytes_free);
    println!("deallocated");
    0
}
