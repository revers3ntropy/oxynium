describe 'External Functions'

expect_err 'SyntaxError' '
    extern fn main();
'
expect_err 'TypeError' '
    extern fn print()
'
expect '' '
    extern fn f();
    extern fn g(p: Int, a: Str = "hi") Str;
'
expect_err 'IoError' '
    extern fn f();
    f()
'
expect_err 'SyntaxError' '
    extern fn f() {}
'