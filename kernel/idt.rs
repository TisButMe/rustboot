use rust::zero;

pub type table = [entry, ..256];

#[packed]
pub struct reg {
    size: u16,
    addr: *mut table,
}

impl reg {
    pub fn new(idt: *mut table) -> reg {
        reg {
            addr: idt,
            size: zero::size_of_val(idt) as u16
        }
    }
}

#[packed]
pub struct entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

pub static PRESENT: u8 = 1 << 7;
pub static PM_32:   u8 = 1 << 3;

pub unsafe fn load(reg_ptr: *const reg) {
    asm!("lidt [$0]" :: "n"(reg_ptr) :: "intel");
}

pub fn entry(proc: u32, sel: u16, flags: u8) -> entry {
    entry {
        addr_lo: (proc & 0xffff) as u16,
        sel: sel,
        zero: 0,
        flags: flags | 0b110,
        addr_hi: (proc >> 16) as u16
    }
}