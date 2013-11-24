#[link(name = "rustboot_x86_64",
       vers = "0.1",
       license = "MIT")];

#[no_std];
#[feature(asm, globs, macro_rules)];

extern {
	fn main();
}

#[lang="start"]
#[no_mangle]
#[no_split_stack]
pub unsafe fn efi_main(sys: *u8) {
    main();
    loop {}
}
