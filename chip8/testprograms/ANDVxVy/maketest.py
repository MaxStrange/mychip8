nop  = b'\x00\x00'
brk  = b'\x00\xA0'
andd = b'\x8A\xE2'  # AND VA and VE and store result in VA
lda  = b'\x6A\x0E'  # Load 0x0E into register VA
lde  = b'\x6E\x03'  # Load 0x03 into register VE
with open("andvxvytest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x0E into register VA
    f.write(lde)   # 0x0202  <-- Load the byte 0x03 into register VE
    f.write(andd)  # 0x0204  <-- Store VA & VE in VA => (0x0E & 0x03)
    f.write(brk)

# 0x0E = 0b0000 1110
# 0x03 = 0b0000 0011
# ------------------
# 0x02 = 0b0000 0010
