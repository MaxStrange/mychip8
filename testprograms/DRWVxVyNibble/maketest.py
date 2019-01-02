nop = b'\x00\x00'
brk = b'\x00\xA0'
ldi = b'\xA2\x40'  # Set I to 0x0240
ld0 = b'\x60\xFF'  # Load 0xFF into V0
ld1 = b'\x61\xA5'  # Load 0xA5 into V1
ld2 = b'\x62\x9D'  # etc.
ld3 = b'\x63\xFF'
ld4 = b'\x64\x81'
ld5 = b'\x65\xFF'
ld6 = b'\x66\x08'
ld7 = b'\x67\x3E'
ld8 = b'\x68\x7F'
ld9 = b'\x69\xEF'
lda = b'\x6A\xFF'
ldb = b'\x6B\xFF'
ldc = b'\x6C\xFF'
sto = b'\xFC\x55'  # Store V0 through VC into memory starting at I
dr1 = b'\xDD\xE4'  # Draw 4-byte sprite at coordinate (VD, VE)
dr2 = b'\xDD\xE2'  # Ditto for a 2-byte sprite
dr3 = b'\xDD\xE3'  # Ditto for a 3-byte sprite
dr4 = b'\xDD\xE4'  # Ditto for a 4-byte sprite
ldx1 = b'\x6D\x35' # Load VD with 0x35 (x-coordinate for first sprite)
ldy1 = b'\x6E\x35' # Load VE with 0x35 (y-coordinate for first sprite)
ldx2 = b'\x6D\x90' # Load VD with 0x90 (x-coordinate for second sprite)
ldy2 = b'\x6E\x95' # Load VE with 0x95 (y-coordinate for second sprite)
ldx3 = b'\x6D\xC0' # Load VD with 0xC0 (x-coordinate for third sprite)
ldy3 = b'\x6E\xC1' # Load VE with 0xC1 (y-coordinate for third sprite)
ldx4 = b'\x6D\xC0' # Load VD with 0xC0 (x-coordinate for fourth sprite)
ldy4 = b'\x6E\xC0' # Load VE with 0xC0 (y-coordinate for fourth sprite)
ldi2 = b'\xA2\x48' # Load I with 0x248 (the second sprite's starting address)
ldi3 = b'\xA2\x4C' # Load I with 0x24C (the third sprite's starting address)
ldi4 = b'\xA2\x52' # Load I with 0x252 (the fourth sprite's starting address)
with open("drwvxvynibbletest.bin", 'wb') as f:
    f.write(ldi)   # 0x0200  <-- Set I to where we will dump the sprites
    f.write(ld0)   # 0x0202  <-- Load V0 with sprite portion 0
    f.write(ld1)   # 0x0204  <-- ditto for V1
    f.write(ld2)   # 0x0206  <-- V2
    f.write(ld3)   # 0x0208  <-- V3
    f.write(ld4)   # 0x020A  <-- V4
    f.write(ld5)   # 0x020C  <-- V5
    f.write(ld6)   # 0x020E  <-- V6
    f.write(ld7)   # 0x0210  <-- V7
    f.write(ld8)   # 0x0212  <-- V8
    f.write(ld9)   # 0x0214  <-- V9
    f.write(lda)   # 0x0216  <-- VA
    f.write(ldb)   # 0x0218  <-- VB
    f.write(ldc)   # 0x021A  <-- VC
    f.write(sto)   # 0x021C  <-- memcpy the registers V0 through VC into memory at I
    f.write(ldx1)  # 0x021E  <-- load VD with x coordinate for first sprite
    f.write(ldy1)  # 0x0220  <-- load VE with y coordinate for first sprite
    f.write(dr1)   # 0x0222  <-- Draw first sprite at x, y
    f.write(ldx2)  # 0x0224  <-- load VD with x coordinate for second sprite
    f.write(ldy2)  # 0x0226  <-- load VE with y coordinate for second sprite
    f.write(ldi2)  # 0x0228  <-- load I with memory address for second sprite
    f.write(dr2)   # 0x022A  <-- Draw second sprite at x, y
    f.write(brk)   # 0x022C  <-- Break to check for collision
    f.write(ldx3)  # 0x022E  <-- load VD with x coordinate for third sprite
    f.write(ldy3)  # 0x0230  <-- load VE with y coordinate for third sprite
    f.write(ldi3)  # 0x0232  <-- load I with memory address for third sprite
    f.write(dr3)   # 0x0234  <-- Draw third sprite at x, y
    f.write(ldx4)  # 0x0236  <-- load VD with x coordinate for fourth sprite
    f.write(ldy4)  # 0x0238  <-- load VE with y coordinate for fourth sprite
    f.write(ldi4)  # 0x023A  <-- load I with memory address for fourth sprite
    f.write(dr4)   # 0x023C  <-- Draw fourth sprite at x, y
    f.write(brk)   # 0x023E  <-- Break to check for collision

    f.write(nop)   # 0x0240  ######## 0b1111 1111 0xFF
    f.write(nop)   # 0x0242  # #  # # 0b1010 0101 0xA5
    f.write(nop)   # 0x0244  #  ### # 0b1001 1101 0x9D
    f.write(nop)   # 0x0246  ######## 0b1111 1111 0xFF

    f.write(nop)   # 0x0248  #      # 0b1000 0001 0x81
    f.write(nop)   # 0x024A  ######## 0b1111 1111 0xFF

    f.write(nop)   # 0x024C      #    0b0000 1000 0x08
    f.write(nop)   # 0x024E    #####  0b0011 1110 0x3E
    f.write(nop)   # 0x0250   ####### 0b0111 1111 0x7F

    f.write(nop)   # 0x0252  ### #### 0b1110 1111 0xEF
    f.write(nop)   # 0x0254  ######## 0b1111 1111 0xFF
    f.write(nop)   # 0x0256  ######## 0b1111 1111 0xFF
    f.write(nop)   # 0x0258  ######## 0b1111 1111 0xFF
