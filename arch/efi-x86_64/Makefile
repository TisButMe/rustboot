RUST_ROOT := /usr/local
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/
OVMF_ROOT := /usr/share/ovmf

TARGET := x86_64-intel-linux

-include ./config.mk

LDS := /usr/lib/elf_x86_64_efi.lds
LIB_GCC := $(shell gcc -print-libgcc-file-name)

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS = -O --target $(TARGET) --lib --emit-llvm

CC := $(LLVM_ROOT)/bin/clang
CFLAGS := -g -O3 -ffreestanding

LD := $(GCC_PREFIX)ld
LDFLAGS := -melf_x86_64

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-x86_64

MODS := $(wildcard ../../*/*.rs) $(wildcard */*.rs)

.PHONY: clean run debug

.DELETE_ON_ERROR:

all: boot/boot.efi

%.o: ../../../%.rs $(MODS)
	$(RUSTC) -O --target $(TARGET) --lib -c ../../main.rs -o boot/main.o

%.o: %.rs
	$(RUSTC) -O --target $(TARGET) --lib -c $^ -o $@
	objcopy -W rust_abi_version $@

boot/boot.elf: boot/main.o boot/boot.o
	ld -nostdlib -znocombreloc -T $(LDS) -shared -Bsymbolic -L/usr/lib /usr/lib/crt0-efi-x86_64.o $^ -o $@ -lefi -lgnuefi $(LIB_GCC)

boot/boot.efi: boot/boot.elf
	objcopy	-j .text -j .sdata -j .data \
			-j .dynamic -j .dynsym  -j .rel \
			-j .rela -j .reloc \
			--target=efi-app-x86_64 $^ $@

run: boot/boot.efi
	$(QEMU) -L $(OVMF_ROOT) -hda fat:boot -no-kvm -m 32

debug: boot/boot.elf boot/boot.efi
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) -L $(OVMF_ROOT) -hda fat:boot -no-kvm -m 64 -s -S"
	tmux split-window -t rustboot "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file boot/boot.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot

clean:
	rm -f boot/*.{o,efi,elf}
