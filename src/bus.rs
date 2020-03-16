//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::{clint::Clint, plic::Plic, uart::Uart};
use crate::exception::Exception;
use crate::memory::Memory;

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and
/// generates per-hart software interrupts and timer
/// interrupts.
pub const CLINT_BASE: usize = 0x200_0000;
/// The size of CLINT.
pub const CLINT_SIZE: usize = 0x10000;

/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all external interrupts in the
/// system to all hart contexts in the system, via the external interrupt source in each hart.
pub const PLIC_BASE: usize = 0xc00_0000;
/// The size of PLIC.
pub const PLIC_SIZE: usize = 0x4000000;

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: usize = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: usize = 0x100;

/// The address which virtIO starts.
pub const VIRTIO_BASE: usize = 0x1000_1000;
/// The size of virtIO.
pub const VIRTIO_SIZE: usize = 0x1000;

/// The address which DRAM starts.
pub const DRAM_BASE: usize = 0x8000_0000;

/// The system bus.
pub struct Bus {
    clint: Clint,
    plic: Plic,
    uart: Uart,
    pub dram: Memory,
}

impl Bus {
    pub fn new() -> Bus {
        Self {
            clint: Clint::new(),
            plic: Plic::new(),
            uart: Uart::new(),
            dram: Memory::new(),
        }
    }

    /// Return the size of source code in the dram.
    pub fn dram_size(&self) -> usize {
        self.dram.size()
    }

    /// Set the binary data to the memory.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        self.dram.set_dram(data);
    }

    /// Write a byte to the system bus.
    pub fn write8(&mut self, addr: usize, val: u8) -> Result<(), Exception> {
        // TODO: Replace the following code with PMP check (Physical Memory Protection)?
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return Ok(self.uart.write(addr, val));
        }
        if DRAM_BASE <= addr {
            return Ok(self.dram.write8(addr - DRAM_BASE, val));
        }
        // TODO: The type of an exception InstructionAccessFault is correct?
        Err(Exception::InstructionAccessFault)
    }

    /// Write 2 bytes to the system bus.
    pub fn write16(&mut self, addr: usize, val: u16) -> Result<(), Exception> {
        if DRAM_BASE <= addr {
            return Ok(self.dram.write16(addr - DRAM_BASE, val));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Write 4 bytes to the system bus.
    pub fn write32(&mut self, addr: usize, val: u32) -> Result<(), Exception> {
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return Ok(self.plic.write(addr, val));
        }
        if DRAM_BASE <= addr {
            return Ok(self.dram.write32(addr - DRAM_BASE, val));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Write 8 bytes to the system bus.
    pub fn write64(&mut self, addr: usize, val: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return Ok(self.clint.write(addr, val));
        }
        if DRAM_BASE <= addr {
            return Ok(self.dram.write64(addr - DRAM_BASE, val));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Read a byte from the system bus.
    pub fn read8(&mut self, addr: usize) -> Result<u8, Exception> {
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return Ok(self.uart.read(addr));
        }
        if DRAM_BASE <= addr {
            return Ok(self.dram.read8(addr - DRAM_BASE));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Read 2 bytes from the system bus.
    pub fn read16(&self, addr: usize) -> Result<u16, Exception> {
        if DRAM_BASE <= addr {
            return Ok(self.dram.read16(addr - DRAM_BASE));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Read 4 bytes from the system bus.
    pub fn read32(&self, addr: usize) -> Result<u32, Exception> {
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return Ok(self.plic.read(addr));
        }
        if DRAM_BASE <= addr {
            return Ok(self.dram.read32(addr - DRAM_BASE));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Read 8 bytes from the system bus.
    pub fn read64(&self, addr: usize) -> Result<u64, Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return Ok(self.clint.read(addr));
        }
        if DRAM_BASE <= addr {
            return Ok(self.dram.read64(addr - DRAM_BASE));
        }
        Err(Exception::InstructionAccessFault)
    }
}
