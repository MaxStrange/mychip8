sys = b'\x00\x00'
brk = b'\x00\xA0'
with open("systest.bin", 'wb') as f:
    f.write(sys)
    f.write(sys)
    f.write(brk)
