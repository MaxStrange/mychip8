nop = b'\x00\x00'
brk = b'\x00\xA0'
lds = b'\xF5\x18' # Load V5 into the sound timer
ld5 = b'\x65\x22' # Load 0x22 (34) into V5
with open("ldstvxtest.bin", 'wb') as f:
    f.write(brk)   # 0x0200  <-- Wait until the test program sets the CPU clock rate
    f.write(ld5)   # 0x0202  <-- Load 0x22 into V5
    f.write(lds)   # 0x0204  <-- Load the sound timer with 0x22
    f.write(nop)   # 0x0206  <-- Wait one clock cycle
    f.write(nop)   # 0x0208  <-- Wait another clock cycle
    f.write(nop)   # 0x020A
    f.write(brk)   # 0x020C  <-- Wait here for test interface
