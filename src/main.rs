#![no_std]
#![no_main]

use core::{panic::PanicInfo};

#[panic_handler]
#[inline(never)]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    // reset io_bank0 & pads_bank0
    unsafe {
        // reset address map 0x40020000
        let reset_ptr: *mut u32 = 0x40020000 as *mut u32;
        let mut value = core::ptr::read_volatile(reset_ptr);
        // io_bank0
        value &= !(1 << 6);
        // pads_bank0
        value &= !(1 << 9);
        core::ptr::write_volatile(reset_ptr, value);
    }
    // wait until io and bank reset
    loop {
        unsafe {
            let is_reset_ptr : *const u32 = 0x40020008 as *const u32;
            let is_reset = core::ptr::read_volatile(is_reset_ptr);
            let io = (is_reset >> 6) & 1;
            let bank = (is_reset >> 9) & 1;
            if io == 1 && bank == 1 { break; }
        }
    }
    unsafe {
        // connect GPIO 25 to SIO
        let io_ptr: *mut u32 = 0x400280cc as *mut u32;
        core::ptr::write_volatile(io_ptr, 5);
        let gpio_oe_set_ptr: *mut u32 = 0xd0000038 as *mut u32;
        core::ptr::write_volatile(gpio_oe_set_ptr, 1 << 25);
        let gpio_out_set_ptr: *mut u32 = 0xd0000018 as *mut u32;
        core::ptr::write_volatile(gpio_out_set_ptr, 1 << 25);
        // remove isolation
        let iso_ptr: *mut u32 = 0x40038068 as *mut u32;
        let mut iso_value = core::ptr::read_volatile(iso_ptr);
        iso_value &= !(1 << 8);
        core::ptr::write_volatile(iso_ptr, iso_value);
    }
    loop {}
}
