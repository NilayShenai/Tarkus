GDT:
    dq 0

.code_seg:


    dw 0x0000


    db 0x00
    dw 0x00


    db 0b10011010


    db 0b10100000


    db 0x00

.data_seg:
    ; limit
    dw 0x0000


    db 0x00
    dw 0x0000


    db 0b10010010


    db 0b10100000


    db 0x00

ALIGN 4
dw 0

GDT_ADDR:
    dw $ - GDT - 1
    dd GDT
