nop = b'\x00\x00'
brk = b'\x00\xA0'
ld = b'\x63\x23'  # Load 0x23 into register V3
se = b'\x33\x23'  # Skip next instruction if V3 == 0x23
with open("sevxbytetest.bin", 'wb') as f:
    f.write(ld)    # 0x0200  <-- Load the byte 0x23 into register V3
    f.write(se)    # 0x0202  <-- Compare V3 against byte 0x23
    f.write(brk)   # 0x0204  <-- If it worked, we should skip this instruction, otherwise we will break here
    f.write(nop)   # 0x0206  <-- If it worked, we should go here. NOP a few times, then break...
    f.write(nop)   # 0x0208
    f.write(nop)   # 0x020A
    f.write(brk)   # 0x020C  <-- ...here
    f.write(nop)   # 0x020E
