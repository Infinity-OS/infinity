#
# This script is used to build Infinity OS and the dependent components
#
# Please make our changes on the config.mak file and not here
#

LDFLAGS = $(NO_AS_NEEDED)
LD = $(prefix)ld

# Include the configuration file
-include config.mak

arch ?= x86_64
target ?= $(arch)-infinity_os
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

rust_os := target/$(target)/debug/libinfinity_os.a
linker_script := arch/$(arch)/linker.ld
grub_cfg := arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard arch/$(arch)/assembly/*.asm)
assembly_object_files := $(patsubst arch/$(arch)/assembly/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

# Qemu variables
QEMU=qemu-system-$(arch)
QEMUFLAGS=-serial mon:stdio -d cpu_reset -d guest_errors
QEMUFLAGS+=-smp 4 -m 1024

.PHONY: all clean run iso cargo

all: $(kernel)

clean:
	@cargo clean
	@rm -rf build

run: $(iso)
	$(QEMU) $(QEMUFLAGS) -cdrom $(iso) -s

debug: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -s -S

r2:
	@r2 -d gdb://localhost:1234

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	@$(LD) $(LDFLAGS) -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

cargo:
	@xargo build --target $(target)

# compile assembly files
build/arch/$(arch)/%.o: arch/$(arch)/assembly/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
