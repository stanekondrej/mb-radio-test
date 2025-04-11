#![no_std]
#![no_main]

use defmt_rtt as _;
use nrf51_hal as _;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = microbit::Board::take().unwrap();

    let radio = radio::Radio::new(board.RADIO);
    unsafe {
        board.NVIC.set_priority(microbit::pac::Interrupt::RADIO, 16);
    }

    radio.enable_ready_ie();

    loop {
        radio.switch_receive();
        defmt::println!("{:?}", radio.check_state());

        radio.switch_transmit();
        defmt::println!("{:?}", radio.check_state());
    }
}
