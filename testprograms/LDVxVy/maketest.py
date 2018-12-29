nop = b'\x00\x00'
brk = b'\x00\xA0'
lda = b'\x6A\x02'  # Load 0x02 into register VA
ldb = b'\x6D\xDD'  # Load 0xDD into register VD
ldx = b'\x8D\xA0'  # Load register VA into VD
with open("ldvxvytest.bin", 'wb') as f:
    f.write(lda)   # 0x0200  <-- Load the byte 0x02 into register VA
    f.write(ldb)   # 0x0202  <-- Load the byte 0xDD into register VD
    f.write(ldx)   # 0x0204  <-- Load VA into VD
    f.write(brk)   # 0x0206  <-- Check to make sure VA and VD are what we expect
