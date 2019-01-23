nop = b'\x00\x00'
brk = b'\x00\xA0'
jp = b'\x12\x08'
with open("jptest.bin", 'wb') as f:
    f.write(jp)    # 0x0200
    f.write(brk)   # 0x0202  <-- program will break here if JP doesn't work
    f.write(nop)   # 0x0204
    f.write(nop)   # 0x0206
    f.write(nop)   # 0x0208
    f.write(brk)   # 0x020A  <-- program should break here
    f.write(brk)   # 0x020C  <-- program will break here at the very least
    f.write(brk)   # 0x020E  <-- or here
    f.write(nop)   # 0x0210
