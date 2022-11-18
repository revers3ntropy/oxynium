section .data
__data_2487179251 dw 1
__data_1964479442 dw 1
__data_75432872 dw 1
__data_2474713272 dw 1
__data_3623990792 dw 1
__data_3579571612 dw 1
section .text
global main
global _start
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
main:
_start:
push __data_3623990792
push __data_1964479442
push __data_2487179251
push __data_2474713272
push __data_75432872
push __data_3579571612
pop rax
pop rbx
mov rdx, [rbx]
add [rax], rdx
push rax
pop rax
pop rbx
mov rdx, [rbx]
add [rax], rdx
push rax
pop rax
pop rbx
mov rdx, [rbx]
add [rax], rdx
push rax
pop rax
pop rbx
mov rdx, [rbx]
add [rax], rdx
push rax
pop rax
pop rbx
mov rdx, [rbx]
add [rax], rdx
push rax
call print_stack
call exit
