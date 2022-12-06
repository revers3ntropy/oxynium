_$_print_digit: ; [number: int, cb: *] => []
    pop rbx ; pop cb
    pop rax ; pop number

    mov rcx, '0'
    add rax, rcx ; add '0' to value to get correct ascii code for digit

    push rax
    mov rsi, rsp

    mov rdx, 2 ; specify length of string
    ; set up syscall
    mov rax, 1
    mov rdi, 1

    syscall

    pop rax
    push rbx ; push callback pointer
    ret

_$_print_char: ; [ascii_code: int, cb: *] => []
    pop rbx ; pop cb
    pop rax ; pop number

    push rax
    mov rsi, rsp

    mov rdx, 2 ; specify length of string
    ; set up syscall
    mov rax, 1
    mov rdi, 1

    syscall

    pop rax
    push rbx ; push callback pointer
    ret

print_str: ; [string: str*, length: int*, cb: *] => []
    pop rbx ; pop cb
    pop rdx ; pop length
    pop rsi ; pop string

    mov rax, 1
    mov rdi, 1

    syscall

    push rbx ; push callback pointer
    ret


print: ; [string: str*, cb: *] => []
       ; prints characters until null byte is reached
    pop rbx ; pop cb
    pop rsi ; pop string
    mov rax, rsi ; copy string pointer

    mov rdx, 0 ; string length

    ; find length of string
    _$_print_find_length:
        mov rcx, [rax]
        cmp rcx, 0
        je _$_print_end_length
        inc rdx
        inc rax
        jmp _$_print_find_length

    _$_print_end_length:
        mov rax, 1
        mov rdi, 1
        syscall
        push rbx
        ret

print_stack_frame_and_exit: ; [...stack: *] => []
    cmp rsp, rbp
    je exit
    push rsp
    call print_int

    add rsp, 8

    call print_nl
    jmp print_stack_frame_and_exit


print_true: ; [cb: *] => []
    push 'e'
    push 'u'
    push 'r'
    push 't'
    call _$_print_char
    call _$_print_char
    call _$_print_char
    call _$_print_char
    ret

print_false: ; [cb: *] => []
    push 'e'
    push 's'
    push 'l'
    push 'a'
    push 'f'
    call _$_print_char
    call _$_print_char
    call _$_print_char
    call _$_print_char
    call _$_print_char
    ret

print_bool: ; [bool: int*, cb: *] => []
    pop rbx ; pop cb
    pop rsi ; pop bool

    mov rax, [rsi]
    cmp rax, 0
    je _$_print_bool_false
    call print_true
    jmp _$_print_bool_end

    _$_print_bool_false:
        call print_false

    _$_print_bool_end:
        push rbx
        call print_nl
        ret


print_int: ; [number: int*, cb: *] => []
           ; prints an 8 byte integer in base 10
           ; src: https://www.javatpoint.com/binary-to-decimal-number-in-c
           ;   while (num > 0) {
           ;        rem = num % 10;
           ;        decimal_num = decimal_num + rem * base;
           ;        num = num / 10;
           ;        base = base * 2;
           ;    }

    pop r8 ; pop cb
    pop r15 ; pop num
    mov r15, [r15]

    mov r10, rsp

    mov r12, 1 ; base
    mov r13, 0 ; remainder
    mov r14, 0 ; digit_count
    mov r11, 0 ; is negative

    cmp r15, 0 ; if num < 0
    jl _$_print_int_negative

    cmp r15, 0 ; if num == 0: break
    jne _$_print_int_start
    mov rax, 0
    push rax
    call _$_print_digit
    jmp _$_print_int_end

    _$_print_int_negative:
        neg r15 ; make num positive
        mov r11, 1 ; set is negative flag
        jmp _$_print_int_start

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
            mov rax, '-'
            push rax
            call _$_print_char

        _$_print_int_end_print_loop:
                ; print digits in reverse of reverse order
                ; (i.e. print digits in correct order)
                ; by popping and printing 'digit_count' stack items
            cmp r14, 0
            jle _$_print_int_return
            dec r14
            call _$_print_digit
            jmp _$_print_int_end_print_loop

    _$_print_int_return:
        pop rax
        push r8
        ret

print_nl:
    ; print NL
    mov rax, 13
    push rax
    call _$_print_char
    ; print CR
    mov rax, 10
    push rax
    call _$_print_char
    ret

add_ints: ; [a: int*, b: int*, cb: *] => [sum: int*]
    pop rdx ; pop cb

    pop rax
    pop rbx
    mov rbx, [rbx]
    add [rax], rbx
    push rax

    push rdx ; push callback pointer
    ret

sub_ints: ; [a: int*, b: int*, cb: *] => [sum: int*]
    pop rdx ; pop cb

    pop rax
    pop rbx
    mov rbx, [rbx]
    sub [rax], rbx
    push rax

    push rdx ; push callback pointer
    ret

exit:
    mov rax, 60
    mov rdi, 0
    syscall

throw:
    mov rax, 60
    mov rdi, 1
    syscall