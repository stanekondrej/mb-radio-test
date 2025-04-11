#![no_std]

use cortex_m::interrupt::CriticalSection;
use microbit::pac::{self, interrupt};

/// Packet length cannot exceed 254 bytes
const _MAX_PACKET_LENGTH: u8 = 254;

// the combined length of S0, LENGTH, S1 and PAYLOAD cannot exceed 254 bytes
pub struct Packet {}

pub struct Radio {
    radio: pac::RADIO,
}

#[derive(defmt::Format)]
pub enum RadioState {
    Disabled,
    RxRu,
    RxIdle,
    Rx,
    RxDisable,
    TxRu,
    TxIdle,
    Tx,
    TxDisable,
}

impl From<u32> for RadioState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::RxRu,
            2 => Self::RxIdle,
            3 => Self::Rx,
            4 => Self::RxDisable,
            9 => Self::TxRu,
            10 => Self::TxIdle,
            11 => Self::Tx,
            12 => Self::TxDisable,
            _ => unreachable!("There aren't any other possible modes for the radio"),
        }
    }
}

impl Radio {
    /// To use the peripheral, use the [`send_packet`] and [`receive_packet`]
    /// functions associtated with this struct. The radio doesn't need to be
    /// explicitly switched on or off; however, if you want to, you can disable
    /// it after you're done with it using [`disable`]
    pub fn new(radio: pac::RADIO) -> Self {
        Self { radio }
    }

    pub fn check_state(&self) -> RadioState {
        let state = self.radio.state.read().bits();
        RadioState::from(state)
    }

    pub fn switch_receive(&self) {
        self.disable();
        unsafe {
            self.radio.tasks_rxen.write(|w| w.bits(0b1));
        }
    }

    pub fn switch_transmit(&self) {
        self.disable();
        unsafe {
            self.radio.tasks_txen.write(|w| w.bits(0b1));
        }
    }

    fn start(&self) {
        unsafe {
            self.radio.tasks_start.write(|w| w.bits(0b1));
        }
    }

    fn stop(&self) {
        unsafe {
            self.radio.tasks_stop.write(|w| w.bits(0b1));
        }
    }

    /// Disable the radio (to save power)
    pub fn disable(&self) {
        unsafe {
            self.radio.tasks_disable.write(|w| w.bits(0b1));
        }
    }

    /// Sets the radio to transmit mode and transmits the [`Packet`] specified
    // TODO: should this require a critical section?
    pub fn send_packet(&self, packet: Packet, _cs: CriticalSection) {
        unsafe {
            self.radio
                .packetptr
                .write(|w| w.bits((&raw const packet).addr() as u32));
        }
        self.switch_transmit();

        self.start();
    }

    pub fn receive_packet(&self) -> Packet {
        todo!()
    }

    /// Enables the `READY` interrupt and event on the peripheral
    pub fn enable_ready_ie(&self) {
        self.radio.intenset.write(|w| w.ready().set());
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::RADIO);
        }
    }
}

#[interrupt]
fn RADIO() {
    defmt::println!("interrupt")
}
