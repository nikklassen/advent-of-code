global _start, buffer, buffer_len
extern parse_int, print_char, print_int, print_str, read

%define nums_len 16

section .data
    checksum_msg db 'Checksum: ', 0
    buffer times 4000 db 0
    buffer_len equ $ - buffer

    nums times 16 dd 0

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

_start:
    call read
    mov rbx, rax

    %define checksum r8

    mov checksum, DWORD 0
    .read_line:
    mov rsi, nums

    cmp [rbx], BYTE 0
    je .end

    push rbx
    push rsi
    call parse_line
    add rsp, 16
    mov rbx, rax

    push rbx ; save the string pointer so we can use rbx in the loop

    .finished_line:
    mov rsi, nums
    mov rdi, nums_len
    dec rdi

    .scan_nums:
    test rdi, rdi ; for rdi = nums_len - 1; rdi >= 0; rdi--
    jl .next_line

    mov rcx, rdi ; for rcx = rdi - 1; rcx >= 0; rcx--
    dec rcx
    .find_divisible:
    test rcx, rcx
    jl .scan_nums_end

    mov rax, rdi
    imul rax, 4
    add rax, rsi
    movzx eax, WORD [rax]

    mov rbx, rcx
    imul rbx, 4
    add rbx, rsi
    movzx ebx, WORD [rbx]

    cdq
    div rbx
    test rdx, rdx
    jne .flip_args

    add checksum, rax
    jmp .done_find_divisible

    .flip_args:
    mov rax, rcx
    imul rax, 4
    add rax, rsi
    movzx eax, WORD [rax]

    mov rbx, rdi
    imul rbx, 4
    add rbx, rsi
    movzx ebx, WORD [rbx]

    cdq
    div rbx
    test rdx, rdx
    jne .done_find_divisible

    add checksum, rax

    .done_find_divisible:
    dec rcx
    jmp .find_divisible

    .scan_nums_end:
    dec rdi
    jmp .scan_nums


    .next_line:
    pop rbx
    inc rbx ; skip the newline
    jmp .read_line

    .end:
    PRINTS checksum_msg
    PRINTI checksum
    PRINTC 0xa

    mov rax, 60 ; exit
    mov rdi, 0
    syscall

parse_line:
    enter 64, 0
    push rbx
    push rsi

    ; args
    ;   - s: string to parse
    ;   - a: output array
    ; output
    ;   - rax: the start of the unparsed string
    mov rbx, [rbp+24]
    mov rsi, [rbp+16]

    .parse:
    push rbx
    call parse_int
    add rsp, 8

    mov [rsi], eax
    add rsi, 4
    mov rbx, rdx ; update position in the string

    .next_token:
    ; parse the next number
    cmp [rbx], BYTE 0xa
    je .finished_line
    cmp [rbx], BYTE 0x20
    jne .parse
    inc rbx
    jmp .next_token

    .finished_line:
    ; set return value
    mov rax, rbx

    pop rsi
    pop rbx
    leave
    ret