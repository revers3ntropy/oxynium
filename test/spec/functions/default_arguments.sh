describe 'Default Arguments to Functions'

expect '123' '
    func f(a: Int, b: Int = 2, c: Int = 3) {
        print(a.str());
        print(b.str());
        print(c.str());
    };
    f(1);
'
expect '14' '
    const a = 1;
    func f(a: Int = a) {
        print(a.str());
    };
    f();
    f(4);
'
expect '44' '
    const u = 1;
    func f(a: Int, b: Int = 5-u) {
        print(a.str());
        print(b.str());
    };
    f(4);
'
expect 'true2hi3' '
    func f(a: Bool, b = 2, c: Str = "hi") Int {
        print(a.str());
        print(b.str());
        print(c);
        return b + 1;
    };
    print(f(true).str());
'
expect_err 'TypeError' '
    func f(a: Int = "") {};
'
expect_err 'TypeError' '
    func f(a: Int = 1, b: Int) {};
'
expect_err 'TypeError' '
    func f(a: Int, b: Int = 1, c: Int) {};
'
expect_err 'SyntaxError' 'func f(true: Bool) {}'
expect_err 'SyntaxError' 'func f(func: Bool) {}'
expect_err 'SyntaxError' 'func f(while: Bool) {}'

