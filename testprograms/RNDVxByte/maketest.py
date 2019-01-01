nop  = b'\x00\x00'
brk  = b'\x00\xA0'
rnd0 = b'\xC0\xFF'  # Load random byte into V0 after ANDing with 0xFF
rnd1 = b'\xC1\xFF'
rnd2 = b'\xC2\xFF'
rnd3 = b'\xC3\xFF'
rnd4 = b'\xC4\xFF'
rnd5 = b'\xC5\xFF'
rnd6 = b'\xC6\xFF'
rnd7 = b'\xC7\xFF'
rnd8 = b'\xC8\xFF'
rnd9 = b'\xC9\xFF'
with open("rndvxbytetest.bin", 'wb') as f:
    f.write(rnd0)   # 0x0200
    f.write(rnd1)   # 0x0202
    f.write(rnd2)   # 0x0204
    f.write(rnd3)   # 0x0206
    f.write(rnd4)   # 0x0208
    f.write(rnd5)   # 0x020A
    f.write(rnd6)   # 0x020C
    f.write(rnd7)   # 0x020E
    f.write(rnd8)   # 0x0210
    f.write(rnd9)   # 0x0212
    f.write(brk)    # 0x0214
    f.write(nop)    # 0x0216
