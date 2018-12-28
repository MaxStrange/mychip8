nop = b'\x00\x00'
brk = b'\x00\xA0'
ld0 = b'\x60\x25'  # Load 0x25 into register V0
ld1 = b'\x61\x0A'  # Load 0x0A into register V1
ld2 = b'\x62\xCC'  # Load 0xCC into register V2
ld3 = b'\x63\xFF'  # Load 0xFF into register V3
ld4 = b'\x64\x10'  # Load 0x10 into register V4
ld5 = b'\x65\x11'  # Load 0x11 into register V5
ld6 = b'\x66\x22'  # Load 0x22 into register V6
ld7 = b'\x67\x23'  # Load 0x23 into register V7
ld8 = b'\x68\x85'  # Load 0x85 into register V8
ld9 = b'\x69\x09'  # Load 0x09 into register V9
lda = b'\x6A\xAE'  # Load 0xAE into register VA
ldb = b'\x6B\x0E'  # Load 0x0E into register VB
ldc = b'\x6C\x44'  # Load 0x44 into register VC
ldd = b'\x6D\x35'  # Load 0x35 into register VD
lde = b'\x6E\x15'  # Load 0x15 into register VE
with open("ldvxbytetest.bin", 'wb') as f:
    f.write(ld0)   # 0x0200
    f.write(ld1)   # 0x0202
    f.write(ld2)   # 0x0204
    f.write(ld3)   # 0x0206
    f.write(ld4)   # 0x0208
    f.write(ld5)   # 0x020A
    f.write(ld6)   # 0x020C
    f.write(ld7)   # 0x020E
    f.write(ld8)   # 0x0210
    f.write(ld9)   # 0x0212
    f.write(lda)   # 0x0214
    f.write(ldb)   # 0x0216
    f.write(ldc)   # 0x0218
    f.write(ldd)   # 0x021A
    f.write(lde)   # 0x021C
    f.write(brk)   # 0x021E
    f.write(nop)   # 0x0220
