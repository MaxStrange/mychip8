nop = b'\x00\x00'
brk = b'\x00\xA0'
lde = b'\x63\x07' # Load 0x07 (character 'a') into register V3
skp = b'\xE3\xA1' # Skip next instruction if user is NOT pressing character held in V3
with open("sknpvxtest.bin", 'wb') as f:
    f.write(lde)   # 0x0200  <-- Load the byte 0x07 into register V3
    f.write(brk)   # 0x0202  <-- Wait here until we receive the go signal (we should get some characters on the mockinput pipe before here)
    f.write(skp)   # 0x0204  <-- Now we should read the input pipe and check it for 'a'
    f.write(brk)   # 0x0206  <-- If it worked, we should skip this breakpoint.
    f.write(nop)   # 0x0208  <-- NOP a few times for good measure...
    f.write(nop)   # 0x020A
    f.write(brk)   # 0x020C  <-- Stop here until we check the PC (which should be here, not at 0x0206) and get a character over the pipe.
    f.write(skp)   # 0x020E  <-- Now we should read the input pipe and check it for 'a'
    f.write(brk)   # 0x0210  <-- If it worked, we should break here.
    f.write(brk)   # 0x0212  <-- If it didn't work, we will break here... or at one of the following breakpoints.
    f.write(brk)   # 0x0214
    f.write(brk)   # 0x0216
