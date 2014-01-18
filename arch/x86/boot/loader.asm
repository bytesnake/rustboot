global __morestack
global abort
global memcmp
global memcpy
global malloc
global free
global start

extern main

use16

; entry point
start:
    ; initialize segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax

    ; load Rust code into 0x7e00...0x1ffff so we can jump to it later
    mov ax, 65|0x200 ; read 65 sectors (32.5 KiB)
    mov ch, 0        ; cylinder & 0xff
    mov cl, 2        ; sector | ((cylinder >> 2) & 0xc0)
    xor dx, dx       ; head
    mov bx, 0x7e00   ; read buffer
    int 0x13
    jc error
    ; load the rest into 3 segments starting at 0x10000
    mov si, 67 ; starting with sector 67
    xor di, di ; and memory segment in di
.loop:
    ; sector 67 + i*128 copied to 0x10000 + i*0x10000
    add di, 0x1000 ; di += 0x10000 >> 4
    mov es, di     ; es = di (destination segment)
    mov ax, si ; ax = sector number
    mov bl, 18
    div bl     ; ax /= 18
    xor bx, bx ; bx = 0 (destination = di * 16 + 0)
    mov dh, al
    mov ch, al
    shr ch, 1  ; ch = (si / 18) >> 1
    and dh, 1  ; dh = (si / 18) & 1
    mov cl, ah ; cl = si % 18
    mov ax, 128|0x200 ; read 128 sectors (64 KiB)
    int 0x13          ; disk read [2]
    jc error
    add si, 128
    cmp di, 0x3000 ; while di != 0x3000
    jne .loop

    ; load protected mode GDT and a null IDT
    cli         ; disable interrupts by clearing a flag [3]
    lgdt [gdtr]
    lidt [idtr]
    ; set protected mode bit of cr0 [4]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    ; far jump to load CS with 32-bit segment 1 (code) [5][6]
    jmp (1 << 3):protected_mode

error:
    mov bx, ax
    mov si, .msg
.loop:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp .loop
.done:
    jmp $
    .msg db "could not read disk", 0

use32
protected_mode:
    ; load all the other segments with 32-bit segment 2 (data)
    mov eax, 2 << 3
    mov ds, eax
    mov es, eax
    mov fs, eax
    mov gs, eax
    mov ss, eax
    ; set up stack
    mov eax, 0x7bff
    mov esp, eax
    ; clear the screen
    mov edi, 0xb8000
    mov ecx, 80*25*2
    mov al, 0
    rep stosb
    ; rust functions compare esp against [gs:0x30] as a sort of stack guard thing
    ; as long as we set [gs:0x30] to dword 0, it should be ok
    mov [gs:0x30], dword 0
    ; jump into Rust
    call main
abort:
__morestack:
memcmp:
memcpy:
malloc:
free:
    jmp $

gdtr:
    dw (gdt_end - gdt) + 1  ; size
    dd gdt                  ; offset

idtr: ; null IDT register
    dw 0
    dd 0

gdt:
    ; null entry
    dq 0
    ; code entry
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10011010   ; access byte - code
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
    ; data entry
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10010010   ; access byte - data
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
gdt_end:

; half kilobyte sized sector ends with magic value 0x55 0xaa
times 510-($-$$) db 0   ; fill unused space with zeros
db 0x55
db 0xaa

%include "memset.asm"
