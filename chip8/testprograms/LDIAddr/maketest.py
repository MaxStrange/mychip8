nop = b'\x00\x00'
brk = b'\x00\xA0'
ldi = b'\xA2\x1E'  # Load the value 0x021E into register I
with open("ldiaddrtest.bin", 'wb') as f:
    f.write(ldi)   # 0x0200 <-- Just load 0x021E into I and check
    f.write(brk)   # 0x0202
