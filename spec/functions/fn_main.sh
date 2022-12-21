describe 'main function'

expect 'hi' '
    fn main() {
        print("hi");
    };
'
expect_err 'SyntaxError' '
    fn f () {};
    fn main() {
        print("hi");
    };
    f();
'
expect_err 'SyntaxError' '
    fn f () {};
    f();
    fn main() {
        print("hi");
    };
'
expect_err 'SyntaxError' '
    fn main() {
        print("hi");
    };
    if true {};
'
expect_err 'SyntaxError' '
    if false {};
    fn main() {
        print("hi");
    };
'
expect 'Hello' '
    const s = "Hello";
    fn main() {
        print(s);
    };
'
expect '16' '
    const s = 16;
    fn main() {
        print(s.str());
    };
'
expect_err 'TypeError' '
    fn main(a: Int) {}
'
expect_err 'TypeError' '
    fn main() {
        return "hi"
    }
'
expect_err 'TypeError' '
    fn main() {
        return 1;
    };
'
expect_err 'TypeError' '
    fn main(): Str {};
'
expect_err 'SyntaxError' '
    fn main();
'

describe 'External Functions'

expect_err 'SyntaxError' '
    extern fn main();
'
expect_err 'TypeError' '
    extern fn print()
'
expect '' '
    extern fn f();
    extern fn g(p: Int, a: Str = "hi"): Str;
'
expect_err 'IoError' '
    extern fn f();
    f()
'
expect_err 'SyntaxError' '
    extern fn f() {}
'
