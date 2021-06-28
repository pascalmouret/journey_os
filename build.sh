cargo build --target=x86_64-rust_os.json
clang --target=x86_64-pc-none-elf -nostdlib -ffreestanding -c boot.s
/usr/local/opt/llvm/bin/ld.lld --script=linker.ld -o rustos.bin -v boot.o target/x86_64-rust_os/debug/librustos_static.a

mkdir -p isodir/boot/grub
cp rustos.bin isodir/boot/rustos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue --sparc-boot -o rustos.iso isodir
