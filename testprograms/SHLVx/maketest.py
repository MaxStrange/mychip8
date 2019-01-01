nop  = b'\x00\x00'
brk  = b'\x00\xA0'
shl1 = b'\x8A\xEE'  # Shift VA's contents left one
shl2 = b'\x8B\xCE'  # Shift VB's contents left one
lda  = b'\x6A\x0E'  # Load 0x0E into register VA
ldb  = b'\x6B\xFD'  # Load 0xFD into register VB
with open("shlvxtest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x0E into register VA
    f.write(nop)   # 0x0202
    f.write(shl1)  # 0x0204  <-- VA = VA << 1, VF = 0
    f.write(brk)   # 0x0206  <-- Break here to check if VA == 0x1C and VF == 0
    f.write(ldb)   # 0x0208  <-- Load the byte 0xFD into register VB
    f.write(nop)   # 0x020A
    f.write(shl2)  # 0x020C  <-- VB = VB << 1, VF = 1
    f.write(brk)   # 0x020E  <-- Break here to check if VA == 0xFA and VF == 1

# 0x0E = 0b0000 1110 --> 0b0001 1100 = 0x1C, with VF = 0

# 0xFD = 0b1111 1101 --> 0b1111 1010 = 0xFA, with VF = 1
