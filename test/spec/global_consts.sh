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
expect '' '
    const db = 1;
    const $ = 1;
    const _ = 1;
    const and = 1;
    const at = 1;
    const byte = 1;
    const eq = 1;
    const include = 1;
    const DB = 1;
    const DD = 1;
    const Dd = 1;
    const dd = 1;
    const DQ = 1;
'