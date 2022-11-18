print_digit: ; [size: int, number: int*, cb: *] => []
    pop rbx ; pop cb

    pop rdx ; pop size
    pop rsi ; pop number
    mov rax, '0'
    add [rsi], rax ; add '0' to value to get correct ascii code for digit

    ; set up syscall
    mov rax, 1
    mov rdi, 1

    push rbx ; push callback pointer

    syscall
    ret

print_char: ; [size: int, ascii_code: int*, cb: *] => []
    pop rbx ; pop callback

    pop rdx ; pop size
    pop rsi ; pop ascii_code
    ; set up syscall
    mov rax, 1
    mov rdi, 1

    push rbx ; push callback pointer

    syscall
    ret

print_stack: ; [value: any, cb: *] => []
             ; prints the last element on the stack as a digit
             ; assuming size 8 bytes
    pop rdi
    pop rax
    push rdi

    push rax
    push 8
    call print_digit

    ret

exit:
    mov rax, 60
    mov rdi, 0
    syscall