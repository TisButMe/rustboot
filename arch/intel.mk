RUST_ROOT := /usr/local
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS = -O --target $(TARGET) --lib --emit-llvm

CC := $(LLVM_ROOT)/bin/clang
CFLAGS := -g -O3 -ffreestanding

LD := $(GCC_PREFIX)ld

GDB := $(GCC_PREFIX)gdb
ASM := nasm
OBJCOPY := $(GCC_PREFIX)objcopy

MODS := $(wildcard ../../*/*.rs) $(wildcard */*.rs)

.PHONY: clean run debug

.DELETE_ON_ERROR:

%.o: %.bc
	$(CC) $(CFLAGS) -c $^ -o $@

clean:
	rm -f *.bin *.o *.img
