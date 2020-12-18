#!/bin/sh

sdir=`dirname $0`
cat "$sdir"/README.md

# Check for QEMU
if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    QEMU=qemu-system-x86_64
else
    cat <<EOF
Error: unable to find qemu-system-x86_64.

1. Try installing the corresponding qemu-system package from your
   distribution.

2. Or compile QEMU from source:
   git clone https://gitlab.com/qemu-project/qemu.git
   cd qemu && ./configure --target-list=x86_64-softmmu && make
   export PATH=\$PWD/build:\$PATH
EOF
    exit 1
fi

export QEMU
exec $QEMU -drive format=raw,file=gameoflife.bin
