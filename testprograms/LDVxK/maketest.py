brk = b'\x00\xA0'
ldk = b'\xF2\x0A'  # Wait for a key to be pressed, then store that key in V2
with open("ldvxktest.bin", 'wb') as f:
    f.write(ldk)   # 0x0200  <-- Wait until the debug interface sends a key
    f.write(brk)   # 0x0202  <-- Check that V2 stores the key we were sent
