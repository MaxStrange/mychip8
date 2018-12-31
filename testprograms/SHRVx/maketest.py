nop  = b'\x00\x00'
brk  = b'\x00\xA0'
shr1 = b'\x8A\xE6'  # Shift VA's contents right one
shr2 = b'\x8B\xC6'  # Shift VB's contents right one
lda  = b'\x6A\x0E'  # Load 0x0E into register VA
ldb  = b'\x6B\xFD'  # Load 0xFD into register VB
with open("shrvxtest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x0E into register VA
    f.write(nop)   # 0x0202
    f.write(shr1)  # 0x0204  <-- VA = VA >> 1, VF = 0
    f.write(brk)   # 0x0206  <-- Break here to check if VA == 0x07 and VF == 0
    f.write(ldb)   # 0x0208  <-- Load the byte 0xFD into register VB
    f.write(nop)   # 0x020A
    f.write(shr2)  # 0x020C  <-- VB = VB >> 1, VF = 1
    f.write(brk)   # 0x020E  <-- Break here to check if VA == 0xFE and VF == 1

# 0x0E = 0b0000 1110 --> 0b0000 0111 = 0x07, with VF = 0

# 0xFD = 0b1111 1101 --> 0b0111 1110 = 0x7E, with VF = 1
