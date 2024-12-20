
src:=src
target:=os

LINKER:=ld.lld
ASSEMBLER:=nasm


all: $(target) $(target)/boot.bin

$(target):
	mkdir -p $(target)

$(target)/boot.o: $(src)/boot.asm $(src)/gdt.asm $(src)/paging.asm $(src)/load_kernel.asm $(src)/ata.asm
	$(ASSEMBLER) -f elf64 $< -o $@

$(target)/libordinal_system.a: $(src)/*.rs
	cargo build --release
	cp target/custom_target/release/libordinal_system.a $(target)/libordinal_system.a

$(target)/boot.bin: linker.ld $(target)/boot.o $(target)/libordinal_system.a
	$(LINKER) -T linker.ld -o $(target)/boot.bin --oformat=binary
	qemu-img resize $(target)/boot.bin 2M

run: all
	qemu-system-x86_64 -d in_asm -hda $(target)/boot.bin --no-reboot --no-shutdown

debug:
	qemu-system-x86_64 -s -S -d in_asm -hda $(target)/boot.bin --no-reboot --no-shutdown

.PHONY: all clean run

clean:
	rm -rf $(target)
