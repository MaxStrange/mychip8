nop  = b'\x00\x00'
brk  = b'\x00\xA0'
sub1 = b'\x8A\xE5'  # SUB VA and VE and store result in VA and store no-borrow in VF
sub2 = b'\x8B\xC5'  # SUB VB and VC and store result in VB and store no-borrow in VF
lda  = b'\x6A\x0E'  # Load 0x0E into register VA
lde  = b'\x6E\x03'  # Load 0x03 into register VE
ldb  = b'\x6B\x0D'  # Load 0x0D into register VB
ldc  = b'\x6C\xEA'  # Load 0xEA into register VC
with open("subvxvytest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x0E into register VA
    f.write(lde)   # 0x0202  <-- Load the byte 0x03 into register VE
    f.write(sub1)  # 0x0204  <-- Store VA - VE in VA => (0x0E - 0x03)
    f.write(brk)   # 0x0206  <-- Break here to check if VA == 0x0B and VF == 1
    f.write(ldb)   # 0x0208  <-- Load the byte 0x0D into register VB
    f.write(ldc)   # 0x020A  <-- Load the byte 0xEA into register VC
    f.write(sub2)  # 0x020C  <-- Store VB - VC in VB => (0x0D - 0xEA)
    f.write(brk)   # 0x020E  <-- Break here to check if VB == 0xDD and VF == 0

# 0x0E = 0b0000 1110
# 0x03 = 0b0000 0011
# ------------------
# 0x0B   -> 0x0B no borrow

# 0x0D = 0b1111 1101
# 0xEA = 0b1110 1100
# ------------------
# 0xFFDD -> 0xDD with borrow
