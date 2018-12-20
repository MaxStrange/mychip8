nop = b'\x00\x00'
jmp = b'\x12\x00'
with open("crashtest.bin", 'wb') as f:
    f.write(nop)
    f.write(nop)
    f.write(nop)
    f.write(nop)
    f.write(nop)
    f.write(nop)
    f.write(nop)
    f.write(nop)
    #f.write(jmp)
