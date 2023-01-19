    extern malloc, memset, memcpy, free
    extern sprintf
    extern time

_$_print_digit: ; [number: int, cb: *]  => Void
    push rbp
    mov rbp, rsp

    add qword [rbp+16], '0' ; convert to ascii code

    mov rsi, rbp
    add rsi, 16 ; rsi points to ascii code

    mov rdx, 2 ; specify length of string (64 bit char)
    ; set up syscall
    mov rax, 1
    mov rdi, 1

    syscall

    mov rsp, rbp
    pop rbp
    ret

_$_print_char: ; [ascii_code: int, cb: *] => Void
    mov rsi, rsp
    add rsi, 8 ; rsi points to ascii code

    mov rdx, 8 ; specify length of string
    ; set up syscall
    mov rax, 1
    mov rdi, 1
    syscall

    mov rsp, rbp
    pop rbp
    ret

_$_allocate: ; [size: int, cb: *] => *int
    push rbp
    mov rbp, rsp

    xor rax, rax

    mov rdi, qword [rbp + 16]
    cmp rdi, 0
    jle _$_allocate_end

    ; https://stackoverflow.com/questions/74932257
    ; stack alignment around call to malloc
    push rbp
    mov rbp, rsp
    sub rsp, 32
    and rsp, -16
    call malloc WRT ..plt
    mov rsp, rbp
    pop rbp

    cmp rax, 0 ; if rax is NULL, fail
    je _$_allocate_error

    push rax
    mov rdi, rax
    mov rsi, 0
    mov rdx, qword [rbp + 16]

    push rbp
    mov rbp, rsp
    sub rsp, 32
    and rsp, -16
    call memset WRT ..plt
    mov rsp, rbp
    pop rbp

    pop rax

    _$_allocate_end:
        mov rsp, rbp
        pop rbp
        ret

    _$_allocate_error:
        push _$_alloc_err_msg
        call print
        pop rax
        push 1
        call exit
        jmp _$_allocate_end
