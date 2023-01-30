describe 'Function Declarations'

expect '' 'func a ()'
expect '' 'func a (a: Int, b: Bool, c: Str)'
expect '' 'func a () Void'
expect '' 'func a (a: Int) Str'
expect_err 'TypeError' 'func a (a) Str'
expect_err 'TypeError' 'func a (a)'
expect_err 'SyntaxError' 'func ()'
expect_err 'SyntaxError' 'func 0 ()'
expect_err 'SyntaxError' 'func 0g ()'
expect_err 'TypeError' 'func g (); func g()'
expect_err 'TypeError' 'func g (a: Int, a: Str) {}'
expect_err 'TypeError' 'func g (a: Int, a: Int) {}'
expect '' 'func g (g=1) {}'
expect '' 'func g (g: Str) {}'
expect_err 'SyntaxError' '
    func g() {
        func f() {};
    };
'
expect_err 'SyntaxError' '
    func g() {
        func f() {
            func h() {};
        };
    };
'