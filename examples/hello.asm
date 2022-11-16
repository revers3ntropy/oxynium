section .data
    msg     dw "Hello, world", 10
    msglen equ $ - msg

section .text
    global main
    global _start
main:
_start:
    push msglen
    push msg
    mov rax, 1        ; write(
    mov rdi, 1        ;   STDOUT_FILENO,
    pop rsi           ;   "Hello, world!\n",
    pop rdx           ;   sizeof("Hello, world!\n")
    syscall           ; );

    mov rax, 60       ; exit(
    mov rdi, 0        ;   EXIT_SUCCESS
    syscall           ; );
