global _start, buffer, buffer_len
extern parse_int, print_char, print_int, print_str, read

section .data
    max_msg db 'Max: ', 0
    min_msg db 'Min: ', 0
    checksum_msg db 'Checksum: ', 0
    buffer times 4000 db 0
    buffer_len equ $ - buffer

section .text

%macro PRINTC 1
    mov rax, %1
    call print_char
%endmacro

%macro PRINTI 1
    mov rax, %1
    push rax
    call print_int
    add rsp, 8
%endmacro

%macro PRINTS 1
    mov rax, %1
    push rax
    call print_str
    add rsp, 8
%endmacro

%macro STACK_CALL 2
    mov rax, %2
    push rax
    call %1
    add rsp, 8
%endmacro


_start:
    call read
    mov rbx, rax

    %define checksum [rsp+16]
    sub rsp, 8
    %define max [rsp+8]
    sub rsp, 8
    %define min [rsp]
    sub rsp, 8

    mov checksum, DWORD 0
    .read_line:
    mov max, DWORD 0
    mov min, DWORD -1

    cmp [rbx], BYTE 0
    je .end

    .parse:
    push rbx
    call parse_int
    add rsp, 8

    mov rbx, rdx ; update position in the string

    cmp rax, [rsp+8]
    jle .compare_less
    mov [rsp+8], rax
    .compare_less:
    cmp rax, [rsp]
    jge .next_token
    mov [rsp], rax

    .next_token:
    ; parse the next number
    cmp [rbx], BYTE 0xa
    je .finished_line
    cmp [rbx], BYTE 0x20
    jne .parse
    inc rbx
    jmp .next_token

    .finished_line:
    PRINTS max_msg
    PRINTI max
    PRINTC 0xa

    PRINTS min_msg
    PRINTI min
    PRINTC 0xa

    mov rax, max
    sub rax, min
    add checksum, rax

    inc rbx
    jmp .read_line

    .end:
    PRINTS checksum_msg
    PRINTI checksum
    PRINTC 0xa

    add rsp, 16 ; cleanup max, min

    mov rax, 60 ; exit
    mov rdi, 0
    syscall