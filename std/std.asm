print_digit: ; [number: int, cb: *] => []
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

print_char: ; [ascii_code: int, cb: *] => []
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

print_int: ; [number: int, cb: *] => []
           ; prints an 8 byte integer in base 10
           ; src: https://www.javatpoint.com/binary-to-decimal-number-in-c
           ;   while (num > 0) {
           ;        rem = num % 10;
           ;        decimal_num = decimal_num + rem * base;
           ;        num = num / 10;
           ;        base = base * 2;
           ;    }

    pop r14 ; pop cb
    pop r15 ; pop num

    push r14 ; push cb

    mov r12, 1 ; base
    mov r13, 0 ; remainder
    mov r14, 0 ; digit_count
    mov r11, 0 ; is negative

    cmp r15, 0 ; if num < 0
    jl __print_int_negative

    cmp r15, 0 ; if num == 0, skip loop
    jne __print_int_loop
    push r15
    call print_digit
    jmp __print_int_end

    __print_int_negative:
        neg r15 ; make num positive
        mov r11, 1 ; set is negative flag
        jmp __print_int_loop

    __print_int_loop:
        ; while number > 0
        cmp r15, 0
        jle __print_int_end

        ; digit_count++
        inc r14

        ; remainder = number % 10
        mov rdx, 0
        mov rax, r15
        mov rcx, 10
        idiv rcx
        pop r13
        push rdx ; push decimal digit

        ; print(remainder * base)
        mov rax, r13
        imul rax, r12
        push rax

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

        jmp __print_int_loop

    __print_int_end: ; do the actual printing off the stack
        pop rax ; clean up stack

        cmp r11, 1 ; if is negative
        je __print_int_end_print_negative
        jmp __print_int_end_print_loop

        __print_int_end_print_negative:
            mov rax, '-'
            push rax
            call print_char

        __print_int_end_print_loop:
                ; print digits in reverse of reverse order
                ; (i.e. print digits in correct order)
                ; by popping and printing 'digit_count' stack items
            cmp r14, 0
            jle __print_int_end_end
            dec r14
            call print_digit
            jmp __print_int_end_print_loop

        __print_int_end_end:
            ret

hello_world:
    mov rax, 'H'
    push rax
    call print_char
    ret

print_stack: ; [value: any, cb: *] => []
             ; prints the last element on the stack as a digit
             ; assuming size 8 bytes
    pop rdi
    pop rax
    push rdi

    mov rax, [rax]
    push rax
    call print_int

    ret

print_nl:
    ; print NL
    mov rax, 13
    push rax
    call print_char
    ; print CR
    mov rax, 10
    push rax
    call print_char
    ret

exit:
    mov rax, 60
    mov rdi, 0
    syscall