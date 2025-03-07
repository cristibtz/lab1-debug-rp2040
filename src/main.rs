#![no_std]
#![no_main]
#![panic_handler]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use rp2040_hal as _;

#[entry]
fn main() -> ! {
    hprintln!("x");
    // write the rest of the code here
    loop {}
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hprintln!("panic");
    loop {}
}
