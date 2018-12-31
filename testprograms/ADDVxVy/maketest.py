nop  = b'\x00\x00'
brk  = b'\x00\xA0'
and1 = b'\x8A\xE4'  # ADD VA and VE and store result in VA and store carry in VF
and2 = b'\x8B\xC4'  # ADD VB and VC and store result in VB and store carry in VF
lda  = b'\x6A\x0E'  # Load 0x0E into register VA
lde  = b'\x6E\x03'  # Load 0x03 into register VE
ldb  = b'\x6B\xFD'  # Load 0xFD into register VB
ldc  = b'\x6C\xEA'  # Load 0xEA into register VC
with open("addvxvytest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x0E into register VA
    f.write(lde)   # 0x0202  <-- Load the byte 0x03 into register VE
    f.write(and1)  # 0x0204  <-- Store VA + VE in VA => (0x0E + 0x03)
    f.write(brk)   # 0x0206  <-- Break here to check if VA == 0x11 and VF == 0
    f.write(ldb)   # 0x0208  <-- Load the byte 0xFD into register VB
    f.write(ldc)   # 0x020A  <-- Load the byte 0xEA into register VC
    f.write(and2)  # 0x020C  <-- Store VB + VC in VB => (0xFD + 0xEA)
    f.write(brk)   # 0x020E  <-- Break here to check if VB == 0xE7 and VF == 1

# 0x0E = 0b0000 1110
# 0x03 = 0b0000 0011
# ------------------
# 0x11   -> 0x11 no carry

# 0xFD = 0b1111 1101
# 0xEA = 0b1110 1100
# ------------------
# 0x01E7 -> 0xE7 with carry in VF
