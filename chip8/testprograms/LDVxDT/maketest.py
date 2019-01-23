nop = b'\x00\x00'
brk = b'\x00\xA0'
ldt = b'\xF5\x07' # Load the current delay timer value into V5
ltd = b'\xF5\x15' # Load V5 into the delay timer
ld5 = b'\x65\x22' # Load 0x22 (34) into V5
with open("ldvxdttest.bin", 'wb') as f:
    f.write(brk)   # 0x0200  <-- Wait until the test program sets the CPU clock rate
    f.write(ld5)   # 0x0202  <-- Load 0x22 into V5
    f.write(ltd)   # 0x0204  <-- Load the delay timer with 0x22
    f.write(nop)   # 0x0206  <-- Wait one clock cycle
    f.write(nop)   # 0x0208  <-- Wait another clock cycle
    f.write(nop)   # 0x020A
    f.write(ldt)   # 0x020C  <-- Load the current value of the delay timer into V5 (should be 0x1E)
    f.write(brk)   # 0x020E  <-- Wait here for test interface
