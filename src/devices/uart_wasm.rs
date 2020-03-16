//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART) for WebAssembly. The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use wasm_bindgen::prelude::*;

use crate::bus::{UART_BASE, UART_SIZE};

/// The interrupt request of UART.
pub const UART_IRQ: usize = 10;

/// Receive holding register (for input bytes).
pub const UART_RHR: usize = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: usize = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER: usize = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR: usize = UART_BASE + 2;
/// Interrupt status register.
pub const UART_ISR: usize = UART_BASE + 2;
/// Line control register.
pub const UART_LCR: usize = UART_BASE + 3;
/// Line status register.
/// LSR BIT 1:
/// 0 = no overrun error (normal)
/// 1 = overrun error. A character arived before receive holding register was emptied or if FIFOs are enabled, an overrun error will occur only after the FIFO is full and the next character has been completely received in the shift register. Note that character in the shift register is overwritten, but it is not transferred to the FIFO.
/// LSR BIT 6:
/// 0 = transmitter holding and shift registers are full.
/// 1 = transmit holding register is empty. In FIFO mode this bit is set to one whenever the the transmitter FIFO and transmit shift register are empty.
pub const UART_LSR: usize = UART_BASE + 5;

/// Output a message to the emulator console.
pub fn stdout8(byte: u8) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let buffer = document
        .get_element_by_id("buffer8")
        .expect("should have a element with a `buffer8` id");

    let message = format!("{}", byte as char);
    let span = document
        .create_element("span")
        .expect("span element should be created successfully");
    span.set_inner_html(&message);
    let result = buffer.append_child(&span);
    if result.is_err() {
        panic!("can't append a span node to a buffer node")
    }
}

#[wasm_bindgen]
/// The UART, the size of which is 0x100 (2**8).
pub struct Uart {
    uart: [u8; UART_SIZE],
}

#[wasm_bindgen]
impl Uart {
    pub fn new() -> Self {
        let mut uart = [0; UART_SIZE];
        uart[UART_LSR - UART_BASE] |= 1 << 5;
        Self { uart }
    }

    pub fn read(&mut self, index: usize) -> u8 {
        match index {
            UART_RHR => {
                self.uart[UART_LSR - UART_BASE] &= !1;
                self.uart[index - UART_BASE]
            }
            _ => self.uart[index - UART_BASE],
        }
    }

    pub fn write(&mut self, index: usize, value: u8) {
        match index {
            UART_THR => {
                stdout8(value);
            }
            _ => {
                self.uart[index - UART_BASE] = value;
            }
        }
    }
}
