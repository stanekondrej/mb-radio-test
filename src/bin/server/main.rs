#![no_std]
#![no_main]

use nrf51_hal as _;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
