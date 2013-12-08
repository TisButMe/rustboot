use platform::io;

pub static PF: u8 = 8;
pub static DF: u8 = 14;

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe {
        io::puts(0, expr);
        io::puts(80, file);
        io::puti(80*2, line as int);

        zero::abort();
    }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe {
        io::puts(0, file);
        io::puti(80, line as int);
        io::puti(80*2, index as int);
        io::puti(80*3, len as int);

        zero::abort();
    }
}
*/

#[fixed_stack_segment]
#[inline(never)]
unsafe fn ex14() {
    io::puti(0, 14);
}

#[fixed_stack_segment]
#[inline(never)]
pub unsafe fn page_fault() -> u64 {
    0 as u64
}
