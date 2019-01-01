nop = b'\x00\x00'
brk = b'\x00\xA0'
ldi = b'\xA2\x30'  # Set I to 0x0230
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
dr1 = b'\xDD\xE4'  # Draw 4-byte sprite at coordinate (VD, VE)  # TODO: Add load instructions for VD and VE between each sprite draw
dr2 = b'\xDD\xE2'  # Ditto for a 2-byte sprite  # TODO: Load I with correct sprite location before each draw
dr3 = b'\xDD\xE3'  # Ditto for a 3-byte sprite
dr4 = b'\xDD\xE4'  # Ditto for a 4-byte sprite
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
    f.write(nop)   # 0x021E  <-- Draw first sprite  TODO
    f.write(nop)   # 0x0220  <-- Draw second sprite TODO
    f.wrtie(brk)   # 0x0222  <-- Break here to check for collision
    f.write(nop)   # 0x0224  <-- Draw third sprite   TODO
    f.write(nop)   # 0x0226  <-- Draw fourth sprite  TODO
    f.write(brk)   # 0x0228  <-- Break here to check for collision
    f.write(nop)   # 0x022A
    f.write(nop)   # 0x022C
    f.write(nop)   # 0x022E
    f.write(nop)   # 0x0230  ######## 0b1111 1111 0xFF
    f.write(nop)   # 0x0232  # #  # # 0b1010 0101 0xA5
    f.write(nop)   # 0x0234  #  ### # 0b1001 1101 0x9D
    f.write(nop)   # 0x0236  ######## 0b1111 1111 0xFF

    f.write(nop)   # 0x0238  #      # 0b1000 0001 0x81
    f.write(nop)   # 0x023A  ######## 0b1111 1111 0xFF

    f.write(nop)   # 0x023C      #    0b0000 1000 0x08
    f.write(nop)   # 0x023E    #####  0b0011 1110 0x3E
    f.write(nop)   # 0x0240   ####### 0b0111 1111 0x7F

    f.write(nop)   # 0x0242  ### #### 0b1110 1111 0xEF
    f.write(nop)   # 0x0244  ######## 0b1111 1111 0xFF
    f.write(nop)   # 0x0246  ######## 0b1111 1111 0xFF
    f.write(nop)   # 0x0248  ######## 0b1111 1111 0xFF
