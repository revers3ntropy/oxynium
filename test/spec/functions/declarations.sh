describe 'Function Declarations'

expect '' '
    def a () {}
    def b (a: Int, b: Bool, c: Str) {}
    def c () Void;
    def d (a: Int) Void {}
    def e (a=1,b=1) {}
    def f (a=1) {}
    def g (g: Str) {}
    def h (g: Str, a: Int) {}
'
expect '' '
    // trailing comma
    def e (a: Int,) Str;
    def f (a: Int, b: Int,) Str;
    class C {
        def g (a: Int,) {},
        def h (a: Int, b: Str,) {}
    }
'

expect '' 'fn () {}'

expect_err 'TypeError' 'def a (a) Str {}'
expect_err 'TypeError' 'def a (a) {}'
expect_err 'SyntaxError' 'def 0 () {}'
expect_err 'SyntaxError' 'def f (,) {}'
expect_err 'SyntaxError' 'def 0g () {}'
expect_err 'SyntaxError' 'extern function g (a: Int, a: Int) {}'
expect_err 'TypeError' 'def g () {}; def g() {}'
expect_err 'TypeError' 'def g (a: Int, a: Str) {}'
expect_err 'TypeError' 'def g (a: Int, a: Int) {}'
expect_err 'TypeError' '
    def g() {
        def f() {}
    }
'
expect_err 'TypeError' '
    def g() {
        def f() {
            def h () {}
        }
    };
'
expect_err 'TypeError' '
    class C {
        def f () {
            def h () {}
        }
    }
'
