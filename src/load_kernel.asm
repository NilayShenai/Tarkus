

read_disk_param:
    mov ah, 0x08
    mov dl, 0x80
    int 0x13
load_kernel:
    mov ah, 0x02
    mov al, 10
    mov dh, 0x00
    mov dl, 0x80
    mov ch, 0x00
    mov cl, 0x02
    mov bx, 0x7e00
    int 0x13
load_done:

