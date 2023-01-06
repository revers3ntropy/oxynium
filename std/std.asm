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
    add rdi, 1                ; add space for null byte
    push rdi
    call _$_allocate
    add rsp, 8
    mov r15, rax              ; r15 = string pointer

    mov rax, 0
    mov rdi, 0
    mov rsi, r15
    mov rdx, qword [rbp + 24]
    syscall

    ; remove trailing new line, and null terminate
    mov rax, r15
    add rax, qword [rbp + 24]
    inc rax
    .last_char_loop:
        dec rax
        cmp byte [rax], 0
        je .last_char_loop
        cmp byte [rax], 10
        je .del_last_char
        cmp byte [rax], 13
        je .del_last_char
        jmp .last_char_after_loop

    .del_last_char:
        mov byte [rax], 0
    .last_char_after_loop:

    push r15
    call Str.from_utf8 ; convert to string
                       ; this is necessary because the string
                       ; comes in as utf8, but the internal
                       ; string representation is 'utf64'
                       ; (utf8 with padding on each char up to 64 bits)
    pop r15

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

Int.str: ; [self: Int, cb: *] => String
         ; https://stackoverflow.com/questions/8257714
    push rbp
    mov rbp, rsp

    ; allocate string
    push 64 ; much larger than needed
    call _$_allocate
    add rsp, 8

    mov r15, rax ; save char*
    ; write string to allocated memory using `sprintf(buf, "%lld", n)`
    mov rdi, rax
    mov rsi, _$_sprintf_Int_str
    mov rdx, qword [rbp + 16]
    mov rax, 0
    mov rcx, rsi
    mov r8, 0
    mov r9, 0

    push rbp
    mov rbp, rsp
    sub rsp, 32
    and rsp, -16
    call sprintf WRT ..plt
    mov rsp, rbp
    pop rbp

    mov rax, r15

    push rax
    call Str.from_utf8
    pop rdi

    mov rsp, rbp
    pop rbp
    ret

Str.from_utf8: ; [utf8: char*, cb: *] => char*
               ; converts utf8 to utf64, the encoding used by Str
               ; https://stackoverflow.com/questions/1543613/how-does-utf-8-variable-width-encoding-work

    push rbp
    mov rbp, rsp

    mov r15, qword [rbp + 16] ; utf8
    xor r13, r13              ; number of chars in utf8

    ; count number of chars in utf8
    ._$_find_chars_loop:
        xor rax, rax
        mov al, byte [r15]

        cmp al, 0
        je ._$_find_chars_end

        mov cl, al
        shr cl, 7          ; if the first bit of the char is 0,
        cmp cl, 0          ; then it is a single byte char
        je ._$_1_byte_char ; otherwise, it is a multi-byte char

        mov cl, al         ; 2-byte char starts with 110x xxxx
        shr cl, 5
        cmp cl, 6
        je ._$_2_byte_char

        mov cl, al         ; 3-byte char starts with 1110 xxxx
        shr cl, 4
        cmp cl, 14
        je ._$_3_byte_char
        ; jmp ._$_4_byte_char ; 4-byte char starts with 1111 0xxx
        ; _$_4_byte_char:
            xor rcx, rcx
            mov cl, byte [r15+3]
            shl rcx, 8
            mov cl, byte [r15+2]
            shl rcx, 8
            mov cl, byte [r15+1]
            shl rcx, 8
            mov cl, byte [r15]
            inc r13
            push rcx
            add r15, 4
            jmp ._$_find_chars_loop

        ._$_3_byte_char:
            xor rcx, rcx
            mov cl, byte [r15+2]
            shl rcx, 8
            mov cl, byte [r15+1]
            shl rcx, 8
            mov cl, byte [r15]
            inc r13
            push rcx
            add r15, 3
            jmp ._$_find_chars_loop

        ._$_2_byte_char:
            xor rcx, rcx
            mov cl, byte [r15+1]
            shl rcx, 8
            mov cl, byte [r15]
            inc r13
            push rcx
            add r15, 2
            jmp ._$_find_chars_loop

        ._$_1_byte_char:
            push rax
            inc r13
            inc r15
            jmp ._$_find_chars_loop

    ._$_find_chars_end:

        ; put the string (currently on stack) into a heap allocated array
        push r13 ; save r13 = num characters

        mov rdi, r13
        add rdi, 64 ; add space for null terminator
        imul rdi, 8
        push rdi
        call _$_allocate
        add rsp, 8
        ; rax = return value (pointer to heap allocated char*)

        pop r13 ; restore r13

    ._$_move_loop:
        cmp r13, 0
        jle ._$_move_return
        dec r13
        pop rdx
        mov qword [rax + r13 * 8], rdx
        jmp ._$_move_loop

    ._$_move_return:
        mov rsp, rbp
        pop rbp
        ret

Str.utf8_size: ; [self: Str, cb: *] => Int
    push rbp
    mov rbp, rsp

    mov r15, qword [rbp + 16]

    push r15
    call Str.len
    pop r15

    mov rdx, rax ; rdx = number of characters
    imul rdx, 8  ; rdx = index of last byte
    add rdx, r15 ; rdx = pointer to last byte

    xor rax, rax
    dec r15
    .loop:
        inc r15
        cmp r15, rdx
        jg .end
        cmp byte [r15], 0
        je .loop
        inc rax
        jmp .loop

    .end:
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

    mov rdx, qword [rbp + 16] ; pop string
    mov r15, qword [rbp + 24] ; pop index

    cmp r15, 0
    jl .backwards

    xor rax, rax ; character
    xor rcx, rcx ; index

    .loop:
        mov rax, qword [rdx]
        test rax, rax
        je .end
        add rdx, 8

        cmp rcx, r15
        jge .end

        inc rcx
        jmp .loop

    .backwards:
        neg r15
        mov r14, rdx

        push rdx
        call Str.len
        pop rdx
        imul rax, 8
        add rdx, rax ; rdx = end of string + 1

        imul r15, 8
        sub rdx, r15 ; rdx = end of string - index
        cmp rdx, r14 ; if rdx < r14, index is out of bounds
        jl .end_and_clear

        mov rax, qword [rdx]
        jmp .end

    .end_and_clear:
        xor rax, rax
    .end:
        mov rsp, rbp
        pop rbp
        ret

Str.at_raw: ; [index: int, string: char*, cb: *] => char
            ; returns the character at the given index
            ; does not check if index is out of bounds

    mov rdx, qword [rsp + 8] ; pop string
    mov r15, qword [rsp + 16] ; pop index

    mov rax, qword [rdx + r15 * 8]

    ret

Str._$_op_eq: ; [lhs: char*, rhs: char*, cb: *] => bool
              ; returns true if the strings are equal
    push rbp
    mov rbp, rsp

    mov r14, qword [rbp + 16] ; r14 = lhs
    mov r13, qword [rbp + 24] ; r13 = rhs

    xor rax, rax ; rax = 0 (index)

    .loop:
        mov rcx, qword [r14 + rax * 8] ; lhs[rax]
        mov rdx, qword [r13 + rax * 8] ; rhs[rax]
        cmp rcx, rdx ; lhs[rax] != rhs[rax]
        jne .not_equal

        test rcx, rcx ; lhs[rax] == 0
        jz .are_equal ; lhs[rax] == rhs[rax] == 0

        inc rax ; rax++

        jmp .loop

    .are_equal:
        mov rax, 1
        mov rsp, rbp
        pop rbp
        ret

    .not_equal:
        xor rax, rax
        mov rsp, rbp
        pop rbp
        ret

Str._$_op_neq: ; [lhs: char*, rhs: char*, cb: *] => bool
               ; returns false if the strings are equal
    push rbp
    mov rbp, rsp

    mov r14, qword [rbp + 16] ; r14 = lhs
    mov r13, qword [rbp + 24] ; r13 = rhs

    xor rax, rax ; rax = 0 (index)

    .loop:
        mov rcx, qword [r14 + rax * 8] ; lhs[rax]
        mov rdx, qword [r13 + rax * 8] ; rhs[rax]
        cmp rcx, rdx ; lhs[rax] != rhs[rax]
        jne .not_equal

        test rcx, rcx ; lhs[rax] == 0
        jz .are_equal ; lhs[rax] == rhs[rax] == 0

        inc rax ; rax++

        jmp .loop

    .not_equal:
        mov rax, 1
        mov rsp, rbp
        pop rbp
        ret

    .are_equal:
        xor rax, rax
        mov rsp, rbp
        pop rbp
        ret

Str._$_op_add: ; [other: char*, self: char*, cb: *] => char*
               ; returns a new string that is the concatenation of lhs and rhs
    push rbp
    mov rbp, rsp

    times 3 push 0             ; space for length of self and other,
                               ; and the new string
    push qword [rbp + 16]      ; self
    call Str.len
    add rsp, 8
    mov qword [rbp - 8], rax   ; [rbp - 8] = self.len()

    push qword [rbp + 24]      ; other
    call Str.len
    add rsp, 8
    mov qword [rbp - 16], rax  ; [rbp - 16] = other.len()

    add rax, qword [rbp - 8]   ; rax = other.len() + self.len()
    inc rax                    ; rax = lhs.len() + rhs.len() + 1 (for null terminator)
    imul rax, 8                ; rax = (lhs.len() + rhs.len() + 1) * 8
    push rax
    call _$_allocate           ; rax = malloc((lhs.len() + rhs.len() + 1) * 8)
    add rsp, 8
    mov qword [rbp - 24], rax  ; [rbp - 24] = new string

    mov rdx, qword [rbp - 8]   ; rcx = self.len()
    imul rdx, 8                ; rcx = self.len() * 8
    mov rsi, qword [rbp + 16]  ; rsi = self
    mov rdi, qword [rbp - 24]  ; rdx = new string

    push rbp
    mov rbp, rsp               ; stack is 16-byte aligned
    sub rsp, 32
    and rsp, -16
    call memcpy WRT ..plt      ; memcpy(new string, self, self.len() * 8)
    mov rsp, rbp
    pop rbp

    mov rdx, qword [rbp - 16]  ; rcx = other.len()
    imul rdx, 8                ; rcx = other.len() * 8
    mov rsi, qword [rbp + 24]  ; rsi = other
    mov rdi, qword [rbp - 24]  ; rdx = new string
    mov rax, qword [rbp - 8]   ; rax = other.len()
    imul rax, 8                ; rax = other.len() * 8
    add rdi, rax               ; rdx = new string + other.len() * 8

    push rbp
    mov rbp, rsp               ; stack is 16-byte aligned
    sub rsp, 32
    and rsp, -16
    call memcpy WRT ..plt      ; memcpy(new string, other, other.len() * 8)
    mov rsp, rbp
    pop rbp

    mov rax, qword [rbp - 24]  ; rax = new string
    mov rsp, rbp
    pop rbp
    ret

Char.str: ; [char: char, cb: *] => char*
           ; stringifies a single character
    push rbp
    mov rbp, rsp

    push 16
    call _$_allocate
    add rsp, 8

    mov rdi, qword [rbp + 16]
    mov qword [rax], rdi

    mov rsp, rbp
    pop rbp
    ret


Char._$_op_eq: ; [lhs: char, rhs: char, cb: *] => bool
               ; returns true if the characters are equal
    push rbp
    mov rbp, rsp

    mov rax, qword [rbp + 16]
    cmp rax, qword [rbp + 24]
    sete al

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

Char.from_int:
Char.as_int: ; [char: char, cb: *] => int
             ; returns the integer value of the character,
             ; or converts an integer to a character,
             ; both are the same thing, this is just for type checking
    mov rax, qword [rsp + 8]

    ret

Ptr._$_op_add: ; [ptr: *T, offset: int, cb: *] => *T
               ; adds an offset to a pointer

    mov rax, qword [rsp + 8]
    add rax, qword [rsp + 16]

    ret

Ptr.str: ; [ptr: *T, cb: *] => char*
         ; stringifies a pointer

    push rbp
    mov rbp, rsp

    push qword [rbp + 16]
    call Int.str
    add rsp, 8

    mov rsp, rbp
    pop rbp
    ret

Time.current_seconds: ; [cb: *] => int
					  ; returns the current time in seconds
	push rbp
	mov rbp, rsp

	;push rbp
	;mov rbp, rsp
	xor rdi, rdi
	sub rsp, 32
	and rsp, -16
	call time WRT ..plt ; return time(NULL)
	;mov rsp, rbp
	;pop rbp

	mov rsp, rbp
	pop rbp
	ret

Time.current_microseconds: ; [cb: *] => int
						   ; returns the current time in milliseconds
	push rbp
	mov rbp, rsp

	mov rax, 0 ; TODO: implement this

	mov rsp, rbp
	pop rbp
	ret