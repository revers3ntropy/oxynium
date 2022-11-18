section .data
    n dq 2
      dq 1
section .text

print_fraction:
    pop rbx

    pop rax

    push rbx

    push rax
    push 1
    call print

    add rax, 0
    push rax
    push 1
    call print

    ret

print:
    pop rbx

    pop rdx
    pop rsi
    mov rax, '0'
    add [rsi], rax
    mov rax, 1
    mov rdi, 1

    push rbx

    syscall
    ret

exit:
    mov rax, 60
    mov rdi, 0
    syscall

print_stack:
    pop rdi
    pop rax

    push rdi
    push rax
    push 1

    call print

    ;cmp rsp, rbp
    ;jg print_stack
    ret

global main
global _start
main:
_start:

    push n
    call print_fraction
    ;call print_stack
    call exit