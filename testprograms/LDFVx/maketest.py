nop = b'\x00\x00'
brk = b'\x00\xA0'
ld0 = b'\x60\x00'  # Load 0x00 into V0
ld1 = b'\x60\x01'  # Load 0x01 into V0
ld2 = b'\x60\x02'  # Load 0x02 into V0
ld3 = b'\x60\x03'  # Load 0x03 into V0
ld4 = b'\x60\x04'  # Load 0x04 into V0
ld5 = b'\x60\x05'  # Load 0x05 into V0
ld6 = b'\x60\x06'  # Load 0x06 into V0
ld7 = b'\x60\x07'  # Load 0x07 into V0
ld8 = b'\x60\x08'  # Load 0x08 into V0
ld9 = b'\x60\x09'  # Load 0x09 into V0
lda = b'\x60\x0A'  # Load 0x0A into V0
ldb = b'\x60\x0B'  # Load 0x0B into V0
ldc = b'\x60\x0C'  # Load 0x0C into V0
ldd = b'\x60\x0D'  # Load 0x0D into V0
lde = b'\x60\x0E'  # Load 0x0E into V0
ldf = b'\x60\x0F'  # Load 0x0F into V0
ldi = b'\xF0\x29'  # Load address for sprite found in V0 into I
with open("ldfvxtest.bin", 'wb') as f:
    f.write(ld0)
    f.write(ldi)
    f.write(brk)
    f.write(ld1)
    f.write(ldi)
    f.write(brk)
    f.write(ld2)
    f.write(ldi)
    f.write(brk)
    f.write(ld3)
    f.write(ldi)
    f.write(brk)
    f.write(ld4)
    f.write(ldi)
    f.write(brk)
    f.write(ld5)
    f.write(ldi)
    f.write(brk)
    f.write(ld6)
    f.write(ldi)
    f.write(brk)
    f.write(ld7)
    f.write(ldi)
    f.write(brk)
    f.write(ld8)
    f.write(ldi)
    f.write(brk)
    f.write(ld9)
    f.write(ldi)
    f.write(brk)
    f.write(lda)
    f.write(ldi)
    f.write(brk)
    f.write(ldb)
    f.write(ldi)
    f.write(brk)
    f.write(ldc)
    f.write(ldi)
    f.write(brk)
    f.write(ldd)
    f.write(ldi)
    f.write(brk)
    f.write(lde)
    f.write(ldi)
    f.write(brk)
    f.write(ldf)
    f.write(ldi)
    f.write(brk)
