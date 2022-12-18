describe 'fn input'

expect 'abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ' '
    print(input())
' 'abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ'
expect 'Hello, World!' 'print(input())' 'Hello, World!'
expect 'Љ а ߷ ߬a ߦ' 'print(input())' 'Љ а ߷ ߬a ߦ'
expect '1:2' 'print(input("1:"))' '2'
expect '1:2' 'print(input("1:", 1))' '2ggfdgf'
expect '1:2' 'print(input("1:", 4))' '2'
expect_err 'TypeError' 'input(true)' '2'
expect_err 'TypeError' 'input(4)' '2'
expect_err 'TypeError' 'input("h", true)' '2'