nop = b'\x00\x00'
brk = b'\x00\xA0'
jpv = b'\xB2\x08'  # Jump to address 0x0208 + V0
ld0 = b'\x60\x04'  # Load 0x04 into V0
with open("jpv0addrtest.bin", 'wb') as f:
    f.write(ld0)   # 0x0200  <-- Load 0x04 into V0
    f.write(jpv)   # 0x0202  <-- Jump to 0x0208 + 0x0004 = 0x020C
    f.write(brk)   # 0x0204  <-- If the instruction fails, we'll end up here
    f.write(nop)   # 0x0206
    f.write(nop)   # 0x0208
    f.write(brk)   # 0x020A
    f.write(brk)   # 0x020C  <-- If the instruction succeeds, we'll end up here
    f.write(brk)   # 0x020E
    f.write(nop)   # 0x0210
