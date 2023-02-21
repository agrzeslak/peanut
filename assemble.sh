#!/bin/sh

echo "$1" > tmp.asm
nasm -felf32 tmp.asm
objdump -DCM intel tmp.o
rm tmp.asm tmp.o
