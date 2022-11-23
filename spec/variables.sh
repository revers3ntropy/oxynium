describe 'Accessing Constants'

expect_exec 'true' '1'
expect_exec 'false' '0'


describe 'Defining constants'

expect_exec 'const a = 1; a' '1'
expect_exec '
    const a = 1;
    const b = 2;
    a + b
' '3'
expect_exec '
    const a = "Some String";
    print(a)
' $'Some String\r'