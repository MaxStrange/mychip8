nop = b'\x00\x00'
brk = b'\x00\xA0'
ld0 = b'\x60\x04'  # Load 0x04 into V0
ld1 = b'\x61\x07'  # Load 0x07 into V1
ld2 = b'\x62\x11'  # Load 0x11 into V2
ld3 = b'\x63\xAA'  # Load 0xAA into V3
ld4 = b'\x64\xBC'  # Load 0xBC into V4
ld5 = b'\x65\x00'  # Load 0x00 into V5
ld6 = b'\x66\x97'  # Load 0x97 into V6
ldi = b'\xA3\x34'  # Load 0x0334 into I
sto = b'\xF6\x55'  # Store V0 through V6 into memory at I
se0 = b'\x60\x00'  # Store 0x00 into V0
se1 = b'\x61\x00'  # Store 0x00 into V1
se2 = b'\x62\x00'  # Store 0x00 into V2
se3 = b'\x63\x00'  # Store 0x00 into V3
se4 = b'\x64\x00'  # Store 0x00 into V4
se5 = b'\x65\x00'  # Store 0x00 into V5
se6 = b'\x66\x00'  # Store 0x00 into V6
red = b'\xF6\x65'  # Read memory at I into registers V0 through V6
with open("lditest.bin", 'wb') as f:
    # Load each of the first 7 registers with a value
    f.write(ld0)
    f.write(ld1)
    f.write(ld2)
    f.write(ld3)
    f.write(ld4)
    f.write(ld5)
    f.write(ld6)

    # Load I with a value
    f.write(ldi)

    # Store the first 7 registers into memory at I
    f.write(sto)

    # Reset the first 7 registers back to 0
    f.write(se0)
    f.write(se1)
    f.write(se2)
    f.write(se3)
    f.write(se4)
    f.write(se5)
    f.write(se6)

    # Test to make sure registers are zeroed
    f.write(brk)

    # Read memory at I into V0 through V6
    f.write(red)

    # Test to make sure memory is correct and registers are correct
    f.write(brk)
