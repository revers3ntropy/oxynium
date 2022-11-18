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

exit:
    mov rax, 60
    mov rdi, 0
    syscall