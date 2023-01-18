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

	mov rax, 96
	lea rdi, [rsp - 16]
	xor esi, esi
	syscall
	mov rax, [rdi + 8]

	mov rsp, rbp
	pop rbp
	ret

List.at_raw: ; <T> [idx: int, self: List<T>, cb: *] => T
	     ; returns the element at the given index

	push rbp
	mov rbp, rsp

	mov rax, qword [rbp + 16]       ; rax = self
	mov rax, qword [rax]            ; rax = self.head
	mov rcx, qword [rbp + 24]       ; rcx = idx
	mov rax, qword [rax + rcx * 8]  ; rax = self.head[idx]

	mov rsp, rbp
	pop rbp
	ret

List.push: ; <T> [val: T, self: T*, cb: *] => void
	       ; pushes the given value onto the end of the list
	push rbp
	mov rbp, rsp

	; self.size++;

	; #1: Allocate space for old list and new element

	mov rax, qword [rbp + 16] ; rax = self
	mov rcx, qword [rax + 8]  ; rdx = self.size
	add rcx, 8 				  ; rdx = self.size + 8
	mov qword [rax + 8], rcx  ; self.size = self.size + 8
	push rcx 		          ; push self.size + 8
	call _$_allocate          ; rax = allocate((self.size + 1) * 8)
	add rsp, 8                ; pop self.size + 8

	; #2: Copy old list to new list

	mov rdi, rax ; rdi = new list
	mov rsi, qword [rbp + 16] ; rsi = self
	mov rdx, qword [rsi + 8]  ; rcx = self.size
	mov rsi, qword [rsi]      ; rsi = old_list.head
	sub rdx, 8 			      ; rcx = self.size - 8

	; just before copying, while rsi = old_list.head
	; set the location of the list to the new list
	mov rcx, qword [rbp + 16] ; rcx = self
	mov qword [rcx], rax      ; self.head = new_list

	push rbp
	mov rbp, rsp
	sub rsp, 32
	and rsp, -16
	call memcpy WRT ..plt    ; memcpy(new list, old list, (old list size - 1) * 8)
	mov rsp, rbp
	pop rbp

	; #3: Add new element to new list

	mov rax, qword [rbp + 16]      ; rax = self
	mov rcx, qword [rax + 8]       ; rcx = self.size
	mov rax, qword [rax]           ; rcx = self.head (new head)
	mov rdx, qword [rbp + 24]      ; rdx = val
	mov qword [rax + rcx - 8], rdx ; new_list[self.len() - 1] = val

	mov rsp, rbp
	pop rbp
	ret

Any.eq: ; <A, B> [a: *const, b: *const, cb: *] => bool
		   ; compares two values for equality
	push rbp
	mov rbp, rsp

	xor rax, rax

	mov rcx, qword [rbp + 16]
	mov rcx, qword [rcx]
	cmp rcx, qword [rbp + 24]
	sete al

	mov rsp, rbp
	pop rbp
	ret

Any.str: ; <T> [self: Any<T>, cb: *] => Str
		 ; returns a string representation of the pointer
	push rbp
	mov rbp, rsp

	mov rax, qword [rbp + 16]
	push qword [rax]
	call Int.str
	add rsp, 8

	mov rsp, rbp
	pop rbp
	ret

Ptr.make_from: ; <T> [value: T, self: Ptr<T>, cb: *] => Ptr<T>
			   ; creates a pointer from a value
	push rbp
	mov rbp, rsp

	push 8
	call _$_allocate
	add rsp, 8

	mov rdx, qword [rbp + 24]
	mov qword [rax], rdx

	mov rsp, rbp
	pop rbp
	ret

Ptr.unwrap: ; <T> [self: Ptr<T>, cb: *] => T
			; returns the value of the pointer
	push rbp
	mov rbp, rsp

	mov rax, qword [rbp + 16]
	mov rax, qword [rax]

	mov rsp, rbp
	pop rbp
	ret
