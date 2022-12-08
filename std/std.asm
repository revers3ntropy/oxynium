print_digit: ; [number: int, cb: *]  => Void
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

print_char: ; [ascii_code: int, cb: *] => Void
    push rbp
    mov rbp, rsp

    mov rsi, rbp
    add rsi, 16 ; rsi points to ascii code

    mov rdx, 8 ; specify length of string (64 bit char)
    ; set up syscall
    mov rax, 1
    mov rdi, 1

    syscall

    mov rsp, rbp
    pop rbp
    ret

print: ; [string: char*, cb: *] => Void
       ; prints characters until null byte is reached
    push rbp
    mov rbp, rsp

    mov rsi, qword [rbp+16] ; pop string
    mov rax, rsi ; copy string pointer

    mov rdx, 0 ; string length

    ; find length of string
    _$_print_find_length:
        mov rcx, qword [rax]
        cmp rcx, 0
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

print_stack_frame_and_exit: ; [...stack: *] => Void
    cmp rsp, rbp
    je exit
    push rsp
    call print_int

    add rsp, 8

    call print_nl
    jmp print_stack_frame_and_exit


print_true: ; [cb: *] => Void
    push 't'
    call print_char
    pop rax
    push 'r'
    call print_char
    pop rax
    push 'u'
    call print_char
    pop rax
    push 'e'
    call print_char
    pop rax
    ret

print_false: ; [cb: *] => Void
    push 'f'
    call print_char
    pop rax
    push 'a'
    call print_char
    pop rax
    push 'l'
    call print_char
    pop rax
    push 's'
    call print_char
    pop rax
    push 'e'
    call print_char
    pop rax
    ret

print_bool: ; [bool: int, cb: *] => Void
    push rbp
    mov rbp, rsp

    cmp qword [rbp+16], 0
    je _$_print_bool_false
    call print_true
    jmp _$_print_bool_end

    _$_print_bool_false:
        call print_false

    _$_print_bool_end:
        call print_nl
        mov rsp, rbp
        pop rbp
        ret


print_int: ; [number: int, cb: *] => Void
           ; prints an 8 byte integer in base 10
           ; src: https://www.javatpoint.com/binary-to-decimal-number-in-c
           ;   while (num > 0) {
           ;        rem = num % 10;
           ;        decimal_num = decimal_num + rem * base;
           ;        num = num / 10;
           ;        base = base * 2;
           ;    }

    push rbp
    mov rbp, rsp

    mov r15, qword [rbp+16] ; pop num

    mov r10, rsp

    mov r12, 1 ; base
    mov r13, 0 ; remainder
    mov r14, 0 ; digit_count
    mov r11, 0 ; is negative

    mov rax, -9223372036854775808
    cmp r15, rax
    jle _$_print_int_0

    cmp r15, 0 ; if num < 0
    jl _$_print_int_negative

    cmp r15, 0 ; if num == 0
    jne _$_print_int_start
    ; fall through if num == 0
    _$_print_int_0:
        push '0'
        call print_char
        pop rax
        jmp _$_print_int_end

    _$_print_int_negative:
        neg r15 ; make num positive
        mov r11, 1 ; set is negative flag
        cmp r15, 0 ; if num == 0: break
        jne _$_print_int_start
        jmp _$_print_int_0

    _$_print_int_start:
        push 0
    _$_print_int_loop:
        ; while number > 0
        cmp r15, 0
        jle _$_print_int_end

        ; digit_count++
        inc r14

        ; remainder = number % 10
        mov rdx, 0
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

        jmp _$_print_int_loop

    _$_print_int_end: ; do the actual printing off the stack
        cmp r11, 1 ; r11 is 1 if negative
        je _$_print_int_end_print_negative
        jmp _$_print_int_end_print_loop

        _$_print_int_end_print_negative:
            push '-'
            call print_char
            pop rax

        _$_print_int_end_print_loop:
                ; print digits in reverse of reverse order
                ; (i.e. print digits in correct order)
                ; by popping and printing 'digit_count' stack items
            cmp r14, 0
            jle _$_print_int_return
            dec r14
            call print_digit
            pop rax
            jmp _$_print_int_end_print_loop

    _$_print_int_return:
        mov rsp, rbp
        pop rbp
        ret

print_nl:
    ; print NL
    mov rax, 13
    push rax
    call print_char
    pop rax
    ; print CR
    mov rax, 10
    push rax
    call print_char
    pop rax

exit:
    mov rax, 60
    mov rdi, 0
    syscall

throw:
    mov rax, 60
    mov rdi, 1
    syscall