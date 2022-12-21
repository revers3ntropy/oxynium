describe 'Strings'

expect 'Hi' 'print("Hi")'
expect 'ݫݨݫ' 'print("ݫݨݫ")'
expect 'Љ а ߷ ߬a ߦ' 'print("Љ а ߷ ߬a ߦ")'
expect_err 'SyntaxError' 'print("hi'
expect_err 'SyntaxError' '"hi'


describe 'Escape Sequences in String Literals'

expect $'\t' 'print("\t")'
expect $'\t\t' 'print("\t\t")'
expect '"' 'print("\"")'
expect 'hello "world"' 'print("hello \"world\"")'
expect "'" "print(\"'\")"
expect_err 'SyntaxError' 'print("\0")'
expect_err 'SyntaxError' 'print("\9")'
expect_err 'SyntaxError' 'print("\x")'