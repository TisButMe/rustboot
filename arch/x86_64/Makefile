TARGET := x86_64-intel-linux

-include ./config.mk
-include ../intel.mk

LDFLAGS := -melf_x86_64
QEMU := qemu-system-x86_64

LDS := /usr/lib/elf_x86_64_efi.lds
LIB_GCC := $(shell gcc -print-libgcc-file-name)

all: boot.efi

# %.bc: ../../%.rs $(MODS)
# 	cd ../..; $(RUSTC) $(RUSTCFLAGS) $*.rs -o arch/x86_64/$@

%.o: ../../%.rs $(MODS)
	cd ../..; $(RUSTC) -O --target $(TARGET) --lib -c $*.rs -o arch/x86_64/$@

%.o: %.rs
	$(RUSTC) -O --target $(TARGET) --lib -c $^ -o $@
	objcopy -W rust_abi_version $@

# %.bc: %.rs
	# $(RUSTC) $(RUSTCFLAGS) $^ -o $@

# main.bc: boot.rs $(MODS)
	# cd ../..; $(RUSTC) $(RUSTCFLAGS) $*.rs -o arch/x86_64/main.bc
	# #$(RUSTC) $(RUSTCFLAGS) boot.rs -o boot.bc
	# #$(CC) $(CFLAGS) -c boot.bc main.bc -o $@

boot.efi: main.o boot.o
	ld -nostdlib -znocombreloc -T $(LDS) -shared -Bsymbolic -L/usr/lib /usr/lib/crt0-efi-x86_64.o $^ -o $@ -lefi -lgnuefi $(LIB_GCC)
	objcopy	-j .text -j .sdata -j .data \
			-j .dynamic -j .dynsym  -j .rel \
			-j .rela -j .reloc \
			--target=efi-app-x86_64 $@

run: boot.efi
	$(QEMU) -L /home/piotr/Desktop/src/Boot2Rust/bios -hda fat:. -no-kvm -m 32

