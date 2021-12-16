cargo build --target=x86_64-rust_os.json

mkdir -p isodir/boot/grub
cp target/x86_64-rust_os/debug/rust-os isodir/boot/rustos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue --sparc-boot -o rustos.iso isodir
