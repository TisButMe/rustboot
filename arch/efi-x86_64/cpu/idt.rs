use core::mem::size_of;

pub type table = [entry, ..256];

#[packed]
pub struct reg {
    size: u16,
    addr: *table,
}

impl reg {
    pub unsafe fn new(idt: *table) -> reg {
        reg {
            addr: idt,
            size: size_of::<table>() as u16
        }
    }
}

pub unsafe fn load(reg: *mut reg) {
    asm!("lidt [$0]" :: "A"(reg) :: "intel");
}

#[packed]
pub struct entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_mid: u16,
    addr_hi: u32,
    zero2: u32
}

pub static PRESENT:   u8 = 1 << 7;
// interrupt gate
pub static INTR_GATE: u8 = 14;

impl entry {
    pub fn new(addr: u64, sel: u16, flags: u8) -> entry {
        entry {
            addr_lo: (addr & 0xffff) as u16,
            sel: sel,
            zero: 0,
            flags: flags | 0b110,
            addr_mid: ((addr >> 16) & 0xffff) as u16,
            addr_hi: (addr >> 32) as u32,
            zero2: 0
        }
    }
}
