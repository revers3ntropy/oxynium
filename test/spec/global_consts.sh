describe 'Defining Global Constants'

expect '1' '
    const a = 1;
    print(a.str())
'
expect '3' '
    const a = 1;
    const b = 2;
    print((a + b).str())
'
expect 'Some String' '
    const a = "Some String";
    print(a)
'
expect_err 'TypeError' '
    const a = 1;
    const a = 2
'
expect_err 'SyntaxError' '
    fn f() {
        const a;
    }
'
expect_err 'SyntaxError' '
    fn f() {
        const a = 0;
    }
'