call = b'\x22\x0A'
nop = b'\x00\x00'
ret = b'\x00\xEE'
brk = b'\x00\xA0'
with open("calltest.bin", 'wb') as f:
    f.write(nop)   # 0x0200
    f.write(nop)   # 0x0202
    f.write(call)  # 0x0204
    f.write(brk)   # 0x0206
    f.write(brk)   # 0x0208
    f.write(brk)   # 0x020A
    f.write(brk)   # 0x020B
    f.write(ret)   # 0x020C
    f.write(nop)   # 0x020D
