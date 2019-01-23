nop = b'\x00\x00'
brk = b'\x00\xA0'
ld8 = b'\x68\xD9'  # Load 0xD9 into register V8
ldi = b'\xA3\x21'  # Set I equal to 0x0321
bcd = b'\xF8\x33'  # Store BCD representation of V8 at I through I+2
with open("ldbvxtest.bin", 'wb') as f:
    f.write(ld8)   # 0x0200  <-- Load decimal 217 (0xD9) into V8
    f.write(ldi)   # 0x0202  <-- Set I equal to 0x0321
    f.write(bcd)   # 0x0204  <-- Store 2 at 0x0321, 1 at 0x0322, and 7 at 0x0323
    f.write(brk)   # 0x0206  <-- Break here for testing
