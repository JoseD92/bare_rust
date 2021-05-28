SRC=$(wildcard src/*.rs)

target/helloworld/release/hello_world: src/main.rs
	cargo xbuild --release --target helloworld.json

kernel: target/helloworld/release/hello_world

bootloader/target/x86_64-bootloader/release/bootloader: kernel
	cd bootloader; KERNEL=../target/helloworld/release/hello_world KERNEL_MANIFEST=../Cargo.toml cargo xbuild --release --features binary

bootloader: bootloader/target/x86_64-bootloader/release/bootloader
	~/.cargo/bin/rust-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 bootloader/target/x86_64-bootloader/release/bootloader bootloader/target/x86_64-bootloader/release/bootloader.bin

run: bootloader
	qemu-system-x86_64 -drive format=raw,file=bootloader/target/x86_64-bootloader/release/bootloader.bin

.PHONY: clean
clean:
	-rm -r target
	-rm Cargo.lock
	-rm -r bootloader/target