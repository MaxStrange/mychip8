nop  = b'\x00\x00'
brk  = b'\x00\xA0'
sub1 = b'\x8A\xE7'  # SUBN VA and VE and store result in VA and store no-borrow in VF
sub2 = b'\x8B\xC7'  # SUBN VB and VC and store result in VB and store no-borrow in VF
lda  = b'\x6A\x0E'  # Load 0x0E into register VA
lde  = b'\x6E\x03'  # Load 0x03 into register VE
ldb  = b'\x6B\x0D'  # Load 0x0D into register VB
ldc  = b'\x6C\xEA'  # Load 0xEA into register VC
with open("subnvxvytest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x0E into register VA
    f.write(lde)   # 0x0202  <-- Load the byte 0x03 into register VE
    f.write(sub1)  # 0x0204  <-- Store VE - VA in VA => (0x03 - 0x0E)
    f.write(brk)   # 0x0206  <-- Break here to check if VA == 0x0B and VF == 0
    f.write(ldb)   # 0x0208  <-- Load the byte 0x0D into register VB
    f.write(ldc)   # 0x020A  <-- Load the byte 0xEA into register VC
    f.write(sub2)  # 0x020C  <-- Store VC - VB in VB => (0xEA - 0x0D)
    f.write(brk)   # 0x020E  <-- Break here to check if VB == 0xDD and VF == 1
