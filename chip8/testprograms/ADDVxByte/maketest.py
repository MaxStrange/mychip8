nop = b'\x00\x00'
brk = b'\x00\xA0'
add = b'\x7A\x65'  # Add 0x65 to contents of register VA
ld  = b'\x6A\x02'  # Load 0x02 into register VA
with open("addvxbytetest.bin", 'wb') as f:
    f.write(ld)    # 0x0200  <-- Load the byte 0x65 into register VA
    f.write(add)   # 0x0202  <-- Add 0x02 to VA
    f.write(brk)   # 0x0204  <-- Break here to check VA's contents, which should be 0x02 + 0x65
