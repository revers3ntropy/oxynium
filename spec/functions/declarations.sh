describe 'Function Declarations'

expect '' 'fn a ()'
expect '' 'fn a (a: Int, b: Bool, c: Str)'
expect '' 'fn a () Void'
expect '' 'fn a (a: Int) Str'
expect_err 'TypeError' 'fn a (a) Str'
expect_err 'TypeError' 'fn a (a)'
expect_err 'SyntaxError' 'fn ()'
expect_err 'SyntaxError' 'fn 0 ()'
expect_err 'SyntaxError' 'fn 0g ()'
expect_err 'TypeError' 'fn g (); fn g()'
expect_err 'TypeError' 'fn g (a: Int, a: Str) {}'
expect_err 'TypeError' 'fn g (a: Int, a: Int) {}'
expect '' 'fn g (g=1) {}'
expect '' 'fn g (g: Str) {}'
expect_err 'SyntaxError' '
    fn g() {
        fn f() {};
    };
'
expect_err 'SyntaxError' '
    fn g() {
        fn f() {
            fn h() {};
        };
    };
'