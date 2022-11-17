section .data
    extern printf

    n dd 97
    m dd 2

section .text
    global main
    global _start

print:
    pop rbx

    pop rdx
    pop rsi
    mov rax, 1
    mov rdi, 1

    push rbx

    syscall
    ret

print_stack:
    pop rdi
    pop rax

    push rdi
    push rax
    push 2

    call print

    ;cmp rsp, rbp
    ;jg print_stack
    ret

exit:
    mov rax, 60
    mov rdi, 0
    syscall

main:
_start:
    push n
    pop rax

    push m
    pop rbx

    mov rdx, [rbx]
    add [rax], rdx
    push rax

    call print_stack
    call exit