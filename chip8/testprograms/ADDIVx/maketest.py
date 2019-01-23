nop = b'\x00\x00'
brk = b'\x00\xA0'
lda = b'\x6A\x08'  # Load 0x08 into register VA
ldi = b'\xA2\x06'  # Load 0x206 into I
adi = b'\xFA\x1E'  # Set I equal to VA + I
with open("addivxtest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x07 into register VA
    f.write(ldi)   # 0x0202  <-- Load the byte 0x03 into register VE
    f.write(adi)   # 0x0204  <-- Set I equal to 0x206 + 0x08 = 0x20E
    f.write(brk)
