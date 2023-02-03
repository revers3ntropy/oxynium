describe 'Default Arguments to Functions'

expect '123' '
    def f(a: Int, b: Int = 2, c: Int = 3) {
        print(a.Str());
        print(b.Str());
        print(c.Str());
    };
    f(1);
'
expect '14' '
    const a = 1;
    def f(a: Int = a) {
        print(a.Str());
    };
    f();
    f(4);
'
expect '44' '
    const u = 1;
    def f(a: Int, b: Int = 5-u) {
        print(a.Str());
        print(b.Str());
    };
    f(4);
'
expect 'true2hi3' '
    def f(a: Bool, b = 2, c: Str = "hi") Int {
        print(a.Str());
        print(b.Str());
        print(c);
        return b + 1;
    };
    print(f(true).Str());
'
expect_err 'TypeError' '
    def f(a: Int = "") {};
'
expect_err 'TypeError' '
    def f(a: Int = 1, b: Int) {};
'
expect_err 'TypeError' '
    def f(a: Int, b: Int = 1, c: Int) {};
'
expect_err 'TypeError' '
    def f(b=1, d=4, c: Int) {};
'
expect_err 'SyntaxError' 'def f(true: Bool) {}'
expect_err 'SyntaxError' 'def f(def: Bool) {}'
expect_err 'SyntaxError' 'def f(while: Bool) {}'

