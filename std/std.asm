    extern malloc, _read, memset

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

_$_clear_memory: ; [start: *, size: int, cb: *] => Void
    push rbp
    mov rbp, rsp

    mov r15, qword [rbp + 24] ; r15 = start
    mov r14, qword [rbp + 16]
    add r14, r15 ; r14 = end

    _$_clear_memory_loop:
        cmp r15, r14
        jge _$_clear_memory_end

        ;mov byte [r15], 0
        add r15, 1
        jmp _$_clear_memory_loop

    _$_clear_memory_end:
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
    call malloc WRT ..plt

    test rax, rax ; if rax is nullptr, fail
    jz _$_allocate_error

    mov rdi, rax
    mov rsi, 0
    mov rdx, qword [rbp + 16]
    call memset WRT ..plt

    _$_allocate_end:
        mov rsp, rbp
        pop rbp
        ret

    _$_allocate_error:
        push _$_alloc_err_msg
        call print
        pop rax
        jmp _$_allocate_end


print: ; [string: char*, cb: *] => Void
       ; prints characters until null byte is reached
    push rbp
    mov rbp, rsp

    mov rsi, qword [rbp+16] ; pop string
    mov rax, rsi ; copy string pointer

    xor rdx, rdx ; string length

    ; find length of string
    _$_print_find_length:
        mov rcx, qword [rax]
        test rcx, rcx
        je _$_print_end_length
        inc rdx
        inc rax
        jmp _$_print_find_length

    _$_print_end_length:
        mov rax, 1
        mov rdi, 1
        syscall

        mov rsp, rbp
        pop rbp
        ret

print_nl:
    ; print NL
    push 13
    call _$_print_char
    pop rax

    ; print CR
    push 10
    call _$_print_char
    pop rax

    ret

input: ; [buffer_size: int, prompt: char*, cb: *] => String
       ; reads from stdin until a newline is reached
       ; allocates string to heap to fit input
       ; returns pointer to string in rax
    push rbp
    mov rbp, rsp

    push qword [rbp + 16]
    call print
    pop rax

    mov rdi, qword [rbp + 24] ; buffer_size
    inc rdi ; extra char for null terminator
    call malloc WRT ..plt
    mov r15, rax ; r15 = string pointer

    xor rax, rax
    xor rdi, rdi
    mov rsi, r15
    mov rdx, qword [rbp + 24]
    syscall

    mov rax, r15

    mov rsp, rbp
    pop rbp
    ret

exit:
    push rbp
    mov rbp, rsp

    mov rax, 60
    mov rdi, qword [rbp + 16]
    syscall

    mov rsp, rbp
    pop rbp
    ret

Int.str: ; [number: int, cb: *] => char*
           ; stringifies an 8 byte integer in base 10
           ; src: https://www.javatpoint.com/binary-to-decimal-number-in-c
           ;   while (num > 0) {
           ;        rem = num % 10;
           ;        decimal_num = decimal_num + rem * base;
           ;        num = num / 10;
           ;        base = base * 2;
           ;    }

    push rbp
    mov rbp, rsp

    mov r15, qword [rbp + 16] ; pop num

    mov r10, rsp

    mov r12, 1 ; base
    xor r13, r13 ; remainder
    xor r14, r14 ; digit_count
    xor r11, r11 ; is negative

    mov rax, -9223372036854775808
    cmp r15, rax
    jle _$_Int.str_0

    test r15, r15 ; if num < 0
    jl _$_Int.str_negative

    test r15, r15 ; if num == 0
    jne _$_Int.str_start
    ; fall through if num == 0
    _$_Int.str_0:
        mov rdi, 16
        call malloc WRT ..plt
        mov r12, rax ; r15 = string pointer
        mov qword [r12], '0'
        jmp _$_Int.str_return

    _$_Int.str_negative:
        neg r15 ; make num positive
        mov r11, 1 ; set is negative flag
        test r15, r15 ; if num == 0: break
        jne _$_Int.str_start
        jmp _$_Int.str_0

    _$_Int.str_start:
        push 0
    _$_Int.str_loop:
        ; while number > 0
        test r15, r15
        jle _$_Int.str_end

        ; digit_count++
        inc r14

        ; remainder = number % 10
        xor rdx, rdx
        mov rax, r15
        mov rcx, 10
        idiv rcx
        push rdx ; push decimal digit

        ; print(remainder * base)
        mov rax, 10
        imul rax, r12

        ; number = number / 10
        mov rax, r15
        xor rdx, rdx ; clear rdx
        mov rcx, 10
        idiv rcx
        mov r15, rax

        ; base = base * 2
        mov rax, r12
        imul rax, 2
        mov r12, rax

        jmp _$_Int.str_loop

    _$_Int.str_end: ; move from stack to heap

        push r11 ; save r11
        mov rdi, r14
        add rdi, 2 ; space for '-' sign and null terminator
        imul rdi, 8
        call malloc WRT ..plt
        mov r12, rax ; r12 = string pointer
        pop r11

        xor r13, r13 ; offset from r12

        cmp r11, 1 ; r11 is 1 if negative
        je _$_Int.str_end_print_negative
        jmp _$_Int.str_end_print_loop

        _$_Int.str_end_print_negative:
            mov qword [r12], '-'
            mov r13, 8

        _$_Int.str_end_print_loop:
                ; print digits in reverse of reverse order
                ; (i.e. print digits in correct order)
                ; by popping and printing 'digit_count' stack items
            test r14, r14
            jle _$_Int.str_return
            dec r14
            pop rax
            add rax, '0'
            mov qword [r12 + r13], rax
            add r13, 8
            jmp _$_Int.str_end_print_loop

    _$_Int.str_return:
        mov rax, r12
        mov rsp, rbp
        pop rbp
        ret


Str.len: ; [string: char*, cb: *] => int
         ; returns length of string in rax
    push rbp
    mov rbp, rsp

    mov rdx, qword [rbp + 16] ; pop string

    xor rax, rax ; string length

    ; find length of string
    _$_Str.len_find_length:
        mov rcx, qword [rdx]
        test rcx, rcx
        je _$_Str.len_end
        inc rax
        add rdx, 8
        jmp _$_Str.len_find_length

    _$_Str.len_end:
        mov rsp, rbp
        pop rbp
        ret

Str.at: ; [index: int, string: char*, cb: *] => char
              ; returns the character at the given index
    push rbp
    mov rbp, rsp

    mov r14, qword [rbp + 16] ; r14 = string
    mov r13, qword [rbp + 24] ; r13 = index

    push r14 ; string
    call Str.len
    mov r15, rax ; string.len()
    pop rcx

    xor rax, rax ; either finish as 0 or overridden with char

    cmp r13, r15  ; index >= string.len()
    jge _$_Str.char_at_end

    cmp rax, r14 ; 0 > index
    jg _$_Str.char_at_end

    mov rax, qword [r14 + r13 * 8] ; get char at index

    _$_Str.char_at_end:
        mov rsp, rbp
        pop rbp
        ret


Char.str: ; [char: char, cb: *] => char*
           ; stringifies a single character
    push rbp
    mov rbp, rsp

    push 16
    call _$_allocate
    pop rcx

    mov rdi, qword [rbp+16]
    mov qword [rax], rdi

    mov rsp, rbp
    pop rbp
    ret


Int._$_op_add: ; [a: int, b: int, cb: *] => int
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    add rax, qword [rbp + 24]

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_sub: ; [a: int, b: int, cb: *] => int
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    sub rax, qword [rbp + 24]

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_gt: ; [a: int, b: int, cb: *] => bool
    push rbp
    mov rbp, rsp

    xor rax, rax
    mov rcx, qword [rbp + 16]
    cmp rcx, qword [rbp + 24]
    setg al

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_lt: ; [a: int, b: int, cb: *] => bool
    push rbp
    mov rbp, rsp

    xor rax, rax
    mov rcx, qword [rbp + 16]
    cmp rcx, qword [rbp + 24]
    setl al

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_eq: ; [a: int, b: int, cb: *] => bool
    push rbp
    mov rbp, rsp

    xor rax, rax
    mov rcx, qword [rbp + 16]
    cmp rcx, qword [rbp + 24]
    sete al

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_neq: ; [a: int, b: int, cb: *] => bool
    push rbp
    mov rbp, rsp

    xor rax, rax
    mov rcx, qword [rbp + 16]
    cmp rcx, qword [rbp + 24]
    setne al

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_gte: ; [a: int, b: int, cb: *] => bool
    push rbp
    mov rbp, rsp

    xor rax, rax
    mov rcx, qword [rbp + 16]
    cmp rcx, qword [rbp + 24]
    setge al

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_lte: ; [a: int, b: int, cb: *] => bool
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    cmp rax, qword [rbp + 24]
    setle al

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_mul: ; [a: int, b: int, cb: *] => int
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    imul rax, qword [rbp + 24]

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_div: ; [a: int, b: int, cb: *] => int
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    cqo
    idiv qword [rbp + 24]

    mov rsp, rbp
    pop rbp
    ret

Int._$_op_mod: ; [a: int, b: int, cb: *] => int
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    cqo
    idiv qword [rbp + 24]
    mov rax, rdx

    mov rsp, rbp
    pop rbp
    ret

Bool._$_op_or: ; [a: bool, b: bool, cb: *] => bool
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    or rax, qword [rbp + 24]

    mov rsp, rbp
    pop rbp
    ret

Bool._$_op_and: ; [a: bool, b: bool, cb: *] => bool
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    and rax, qword [rbp + 24]

    mov rsp, rbp
    pop rbp
    ret