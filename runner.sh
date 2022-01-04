SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

mkdir -p $SCRIPT_DIR/target/isodir/boot/grub
cp $1 $SCRIPT_DIR/target/isodir/boot/rustos.bin
cp $SCRIPT_DIR/grub.cfg $SCRIPT_DIR/target/isodir/boot/grub/grub.cfg
grub-mkrescue -o $SCRIPT_DIR/target/rustos.iso $SCRIPT_DIR/target/isodir

qemu-system-x86_64 -cdrom $SCRIPT_DIR/target/rustos.iso -device VGA -monitor stdio
