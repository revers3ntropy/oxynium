
section .data
    n dd 97
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
    push 4

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
    add rax, 0x0
    push rax

    call print_stack

    pop rax

    call exit