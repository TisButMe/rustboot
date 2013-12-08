use core::option::{Option, None};
use platform::io;

pub static IRQ: u8 = 0x20 + 1;

static Layout: &'static str = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

pub static mut keydown: Option<extern fn(char)> = None;

static mut shift: bool = false;
static mut ledstate: u8 = 0;

fn led(state: u8) {
    io::wait(0x64, 2);
    io::out(0x60, 0xEDu8);
    io::wait(0x64, 2);
    unsafe { io::out(0x60, ledstate); }
}

#[fixed_stack_segment]
#[inline(never)]
unsafe fn keypress(code: u32) {
    match (code & 0x7f, code & 0x80 == 0) {
        (0x2A, b)  | (0x36, b)  => unsafe { shift = b },
        (0x3A, false) => led(0b100), // Caps lock
        (0x45, false) => led(0b10), // Number lock
        (0x46, false) => led(0b1), // Scroll lock
        (c, true) => {
            keydown.map(|f| {
                f(Layout[c] as char);
            });
        }
        _ => {}
    }
}

#[fixed_stack_segment]
#[inline(never)]
pub unsafe fn isr_addr() -> u64 {
    let mut code: u32;

    // 0x60, 0x61 are instructions pusha(d), popa(d)
    asm!("jmp skip_isr_addr
      isr_addr_asm:
          .byte 0x60

          xor eax, eax
          in al, 60h"
        : "=A"(code) ::: "intel");
          keypress(code);
    asm!("mov dx, 20h
          mov al, dl
          out dx, al

          .byte 0x61
          iretq
      skip_isr_addr:"
        :::: "intel");

    isr_addr_asm as u64
}

extern "C" { pub fn isr_addr_asm(); }
