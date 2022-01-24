SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SUCCESS_CODE=85 # QEMU shifts exit code by one

mkdir -p $SCRIPT_DIR/target/isodir/boot/grub
cp $1 $SCRIPT_DIR/target/isodir/boot/journey_os.bin
cp $SCRIPT_DIR/grub.cfg $SCRIPT_DIR/target/isodir/boot/grub/grub.cfg
grub-mkrescue -o $SCRIPT_DIR/target/journey_os.iso $SCRIPT_DIR/target/isodir

qemu-system-x86_64 \
  -cdrom $SCRIPT_DIR/target/journey_os.iso \
  -device VGA \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -serial stdio
#  -monitor stdio \

[ $? -eq $SUCCESS_CODE ] && exit 0 || exit 1
