global parse_int, print_int, print_char, print_str, read
extern buffer, buffer_len

parse_int:
    ; returns rax = the parsed integer
    ;         rdx = where it stopped parsing
    enter 64, 0
    push rbx
    push rdi
    push rsi

    ; rsi = accumulator
    ; rbx = input string
    ; rcx = current character
    ; rdi = current power of 10
    mov rbx, [rbp+16]
    mov rsi, 0
    mov rdi, 1

    .read_char:
    movzx rcx, byte [rbx]
    cmp rcx, '0'
    jl .done
    cmp rcx, '9'
    jg .done

    sub rcx, '0'
    mov rax, 10
    mul rsi
    mov rsi, rax
    add rsi, rcx
    inc rbx
    jmp .read_char

    .done:
    mov rax, rsi
    mov rdx, rbx
    pop rsi
    pop rdi
    pop rbx
    leave
    ret

print_int:
    enter 64, 0
    push rbx
    push rdi
    push rsi

    mov rax, [rbp+16]
    mov rbx, 0

    .div_loop:
    inc rbx
    cdq
    mov rsi, 10
    div rsi
    add rdx, '0'
    sub rsp, 1
    mov [rsp], dl
    cmp rax, 0
    jnz .div_loop

    push rbx ; number of digits
    mov rax, rsp
    add rax, 8
    push rax ; start of the string
    call write
    add rsp, 16
    imul rbx, 4 ; remove integers
    add rsp, rax

    pop rsi
    pop rdi
    pop rbx
    leave
    ret

read:
    ; prints the character in rax
    enter 64, 0
    push rdi
    push rsi

    mov rbx, buffer

    mov rax, 0 ; call read
    mov rdi, 0
    mov rsi, rbx
    mov rdx, buffer_len
    syscall

    mov rax, rbx

    pop rsi
    pop rdi
    leave
    ret

print_char:
    enter 64, 0

    push rax
    mov rax, 1
    push rax
    mov rax, rsp
    add rax, 8
    push rax
    call write
    add rsp, 24

    leave
    ret

print_str:
    enter 64, 0

    mov rdx, [rbp+16]
    mov rcx, 0

    .test_null:
    cmp [rdx], BYTE 0
    je .print
    inc rcx
    inc rdx
    jmp .test_null

    .print:
    push rcx
    mov rdx, [rbp+16]
    push rdx
    call write
    add rsp, 16

    leave
    ret

write:
    enter 64, 0
    push rdi
    push rsi

    mov rdx, [rbp+24] ; number of bytes
    mov rsi, [rbp+16] ; address of string
    mov rax, 1
    mov rdi, 1 ; stdout
    syscall

    pop rsi
    pop rdi
    leave
    ret
