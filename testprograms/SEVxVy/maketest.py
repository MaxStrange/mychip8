nop = b'\x00\x00'
brk = b'\x00\xA0'
ld1 = b'\x63\x25'  # Load 0x25 into register V3
ld2 = b'\x64\x25'  # Load 0x25 into register V4
se = b'\x53\x40'   # Skip next instruction if V3 != V4
with open("sevxvytest.bin", 'wb') as f:
    f.write(ld1)   # 0x0200  <-- Load the byte 0x25 into register V3
    f.write(ld2)   # 0x0202  <-- Load the byte 0x25 into register V4
    f.write(se)    # 0x0204  <-- Skip next instruction if V3 == V4
    f.write(brk)   # 0x0206  <-- If it worked, we should skip here. If it didn't, we'll break here.
    f.write(nop)   # 0x0208
    f.write(nop)   # 0x020A
    f.write(brk)   # 0x020C  <-- If it worked, we should end up here after a few NOPs
    f.write(nop)   # 0x020E
