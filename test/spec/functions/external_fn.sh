describe 'External Functions'

expect_err 'SyntaxError' '
    extern def main();
'
expect_err 'TypeError' '
    extern def print()
'
expect '' '
    extern def f();
    extern def g(p: Int, a: Str = "hi") Str;
'
expect_err 'IoError' '
    extern def f();
    f()
'
expect_err 'SyntaxError' '
    extern def f() {}
'