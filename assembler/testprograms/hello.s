LD 3, 0x25 ; Load 0x25 into V3
LD 4, 0x25 ; Load 0x25 into V4
SNE 3, 4   ; Skip the next instruction if V3 does not equal V4
BRK        ; Should break here
NOP
NOP
BRK        ; Should never reach this breakpoint
