
            section .data
                __data_1144646114 dw 1
__data_3158769850 dw 99
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
                push __data_3158769850
push __data_1144646114
   pop rax
   pop rdx
   add rax, rdx
   push rax
                call print_stack
                call exit


        