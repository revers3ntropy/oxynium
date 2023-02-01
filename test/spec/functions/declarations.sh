describe 'Function Declarations'

expect '' '
    func a () {}
    func b (a: Int, b: Bool, c: Str) {}
    func c () Void;
    func d (a: Int) Void {}
    func e (a=1,b=1) {}
    func f (a=1) {}
    func g (g: Str) {}
    func h (g: Str, a: Int) {}
'
expect '' '
    // trailing comma
    func e (a: Int,) Str;
    func f (a: Int, b: Int,) Str;
    class C {
        func g (a: Int,) {}
        func h (a: Int, b: Str,) {}
    }
'
expect_err 'TypeError' 'func a (a) Str {}'
expect_err 'TypeError' 'func a (a) {}'
expect_err 'SyntaxError' 'func () {}'
expect_err 'SyntaxError' 'func 0 () {}'
expect_err 'SyntaxError' 'func f (,) {}'
expect_err 'SyntaxError' 'func 0g () {}'
expect_err 'TypeError' 'func g () {}; func g() {}'
expect_err 'TypeError' 'func g (a: Int, a: Str) {}'
expect_err 'TypeError' 'func g (a: Int, a: Int) {}'
expect_err 'SyntaxError' '
    func g() {
        func f() {}
    }
'
expect_err 'SyntaxError' '
    func g() {
        func f() {
            func h () {}
        }
    };
'
expect_err 'SyntaxError' '
    class C {
        func f () {
            func h () {}
        }
    }
'