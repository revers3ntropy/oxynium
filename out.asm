
            section .data
                __data_173056449 dw 1
__data_2757973983 dw 99
            section .text
            global main
            global _start

            print:
                pop rbx

                pop rdx
                pop rsi
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
                push 2

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
                push __data_2757973983
push __data_173056449
   pop rax
   pop rbx
   mov rdx, [rbx]
   add [rax], rdx
   push rax
                call print_stack
                call exit


        