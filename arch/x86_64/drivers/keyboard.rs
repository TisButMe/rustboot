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
pub unsafe fn isr_addr() -> u32 {
    0 as u32
}

/*#[fixed_stack_segment]
#[inline(never)]
pub unsafe fn isr_addr() {
    let mut ptr: u32;

    // push   ebp
    // mov    ebp,esp
    // push   esi
    // push   eax
    // mov    esi,esp

    asm!(".word 0xa80f
          .word 0xa00f
          .byte 0x06
          .byte 0x1e
          pusha

          xor eax, eax
          in al, 60h
          push eax"
        : "=A"(ptr) :: "esp" : "intel");
          keypress(ptr);
    asm!("pop eax
          mov dx, 20h
          mov al, dl
          out dx, al

          popa
          .byte 0x1f
          .byte 0x07
          .word 0xa10f
          .word 0xa90f
          add esp, 4
          pop esi
          pop ebp
          iretd"
        :::: "intel");
        // add    esp,0x4
        // pop    esi
        // pop    ebp
        // ret
}*/
