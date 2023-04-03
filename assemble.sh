#!/bin/sh

# Used to pass input to the NASM assembler and display the resultant assembly.
# e.g. ./assemble.sh "add al, 0xff"


set -e
echo "$1" > tmp.asm
nasm -felf32 tmp.asm
objdump -DCM intel tmp.o
rm tmp.asm tmp.o
