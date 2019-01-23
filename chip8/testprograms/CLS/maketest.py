cls = b'\x00\xE0'
brk = b'\x00\xA0'
with open("clstest.bin", 'wb') as f:
    f.write(cls)
    f.write(cls)
    f.write(brk)
