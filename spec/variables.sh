describe 'Accessing Constants'

expect 'true' '1'
expect 'false' '0'


describe 'Defining constants'

expect 'const a = 1; a' '1'
expect '
    const a = 1;
    const b = 2;
    a + b
' '3'
expect '
    const a = "Some String";
    print(a)
' $'Some String\r'