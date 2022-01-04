mkdir -p target/isodir/boot/grub
cp $1 target/isodir/boot/rustos.bin
cp grub.cfg target/isodir/boot/grub/grub.cfg
grub-mkrescue -o target/rustos.iso target/isodir

qemu-system-x86_64 -cdrom target/rustos.iso -device VGA -monitor stdio
