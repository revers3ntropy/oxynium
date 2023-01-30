describe 'External Functions'

expect_err 'SyntaxError' '
    extern func main();
'
expect_err 'TypeError' '
    extern func print()
'
expect '' '
    extern func f();
    extern func g(p: Int, a: Str = "hi") Str;
'
expect_err 'IoError' '
    extern func f();
    f()
'
expect_err 'SyntaxError' '
    extern func f() {}
'