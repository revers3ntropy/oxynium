describe 'Default Arguments to Functions'

expect '123' '
    fn f(a: Int, b: Int = 2, c: Int = 3) {
        print(a.str());
        print(b.str());
        print(c.str());
    };
    f(1);
'
expect '14' '
    const a = 1;
    fn f(a: Int = a) {
        print(a.str());
    };
    f();
    f(4);
'
expect '44' '
    const u = 1;
    fn f(a: Int, b: Int = 5-u) {
        print(a.str());
        print(b.str());
    };
    f(4);
'
expect 'true2hi3' '
    fn f(a: Bool, b = 2, c: Str = "hi"): Int {
        print(a.str());
        print(b.str());
        print(c);
        return b + 1;
    };
    print(f(true).str());
'
expect_err 'TypeError' '
    fn f(a: Int = "") {};
'
expect_err 'TypeError' '
    fn f(a: Int = 1, b: Int) {};
'
expect_err 'TypeError' '
    fn f(a: Int, b: Int = 1, c: Int) {};
'
expect_err 'SyntaxError' 'fn f(true: Bool) {}'
expect_err 'SyntaxError' 'fn f(fn: Bool) {}'
expect_err 'SyntaxError' 'fn f(while: Bool) {}'

