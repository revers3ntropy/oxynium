
            section .data
                __data_1219610745 dw "1"
            section .text
            global main
            global _start
            main:
            _start: ; entry point to program
                push 4

                push __data_1219610745

                mov rax, 1
                mov rdi, 1
                pop rsi
                pop rdx
                syscall

                mov rax, 60       ; exit(
                mov rdi, 0        ;   EXIT_SUCCESS
                syscall           ; );

        