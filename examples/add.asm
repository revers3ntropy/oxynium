
section .data
    big_ dw 97
    small_ dw 1
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
    push big_
    pop rax

    push small_
    pop rbx

    add rax, rbx

    push rax

    call print_stack
    call exit