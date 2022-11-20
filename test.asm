section .data
    __anon_data_1 dq 2
    __anon_data_0 dq 3
section .text
    global main
    global _start
print_digit:
    pop rbx
    pop rax
    mov rcx, '0'
    add rax, rcx
    push rax
    mov rsi, rsp
    mov rdx, 2
    mov rax, 1
    mov rdi, 1
    syscall
    pop rax
    push rbx
    ret
print_char:
    pop rbx
    pop rax
    push rax
    mov rsi, rsp
    mov rdx, 2
    mov rax, 1
    mov rdi, 1
    syscall
    pop rax
    push rbx
    ret
print_int:
    pop r14
    pop r15
    push r14
    mov r12, 1
    mov r13, 0
    mov r14, 0
    mov r11, 0
    cmp r15, 0
    jl __print_int_negative
    cmp r15, 0
    jne __print_int_loop
    push r15
    call print_digit
    jmp __print_int_end
__print_int_negative:
    neg r15
    mov r11, 1
    jmp __print_int_loop
__print_int_loop:
    cmp r15, 0
    jle __print_int_end
    inc r14
    mov rdx, 0
    mov rax, r15
    mov rcx, 10
    idiv rcx
    pop r13
    push rdx
    mov rax, r13
    imul rax, r12
    push rax
    mov rax, r15
    xor rdx, rdx
    mov rcx, 10
    idiv rcx
    mov r15, rax
    mov rax, r12
    imul rax, 2
    mov r12, rax
    jmp __print_int_loop
__print_int_end:
    pop rax
    cmp r11, 1
    je __print_int_end_print_negative
    jmp __print_int_end_print_loop
__print_int_end_print_negative:
    mov rax, '-'
    push rax
    call print_char
__print_int_end_print_loop:
    cmp r14, 0
    jle __print_int_end_end
    dec r14
    call print_digit
    jmp __print_int_end_print_loop
__print_int_end_end:
    ret
print_stack:
    pop rdi
    pop rax
    push rdi
    mov rax, [rax]
    push rax
    call print_int
    ret
exit:
    mov rax, 60
    mov rdi, 0
    syscall
main:
_start:
    push __anon_data_0
    push __anon_data_1
    pop rcx
    pop rbx
    mov rbx, [rbx]
    mov rax, [rcx]
    imul rbx
    mov [rcx], rax
    push rcx
    call print_stack
    call exit
