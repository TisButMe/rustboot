use super::drivers::vga;
use super::drivers::keyboard;
use kernel::int;
use core::option::Some;

pub static mut pos: int = 0;

pub unsafe fn seek(offset: int) {
    pos += offset;
    vga::cursor_at(pos as uint);
}

pub unsafe fn write_char(c: char) {
    if c == '\x08' {
        if pos > 0 {
            if pos % 80 == 0 {
                while (*vga::SCREEN)[pos-1].char == 0 {
                    pos -= 1;
                }
            }
            else if pos > 0 {
                if pos > 0 { pos -= 1; }
                (*vga::SCREEN)[pos].char = 0;
            }
        }
    }
    else if c == '\n' {
        seek(80 - pos % 80);
    }
    else {
        (*vga::SCREEN)[pos].char = c as u8;
        pos += 1;
    }

    vga::cursor_at(pos as uint);
}

pub unsafe fn keydown(f: extern fn(char)) {
    keyboard::keydown = Some(f);
}

pub unsafe fn puts(j: int, buf: *u8) {
    let mut i = j;
    let mut curr = buf;
    while *curr != 0 {
        (*vga::SCREEN)[i].char = *curr;
        (*vga::SCREEN)[i].attr = 16;
        i += 1;
        curr = (curr as uint + 1) as *u8;
    }
}

pub unsafe fn puti(j: uint, num: int) {
    let mut i = j;
    int::to_str_bytes(num, 10, |n| {
        (*vga::SCREEN)[i].char = n;
        (*vga::SCREEN)[i].attr = 16;
        i += 1;
    });
}

#[inline(always)]
pub fn out<T>(port: u16, val: T) {
    unsafe {
        asm!("out $0, $1" :: "{al}"(val), "{dx}"(port) :: "intel");
    }
}

pub fn wait(port: u16, mask: u8) {
    unsafe {
        loop {
            let mut val: u8;
            asm!("in $1, $0" : "={al}"(val) : "{dx}"(port) :: "intel");
            if val & mask == 0 { break }
        }
    }
}
