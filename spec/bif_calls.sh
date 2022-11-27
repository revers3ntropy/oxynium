describe '`print` BIF'

expect 'print("Hello, World!")' $'Hello, World!\r'


describe '`print_int` BIF'

expect 'print_int(1)' '1'
expect 'print_int(9*7%3)' '0'
