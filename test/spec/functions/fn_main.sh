describe 'main Function'

expect 'hi' '
    func main() {
        print("hi");
    }
'
expect_err 'TypeError' '
    func main(a: Int) {}
'
expect_err 'TypeError' '
    func main() {
        return "hi"
    }
'
expect_err 'TypeError' '
    func main() {
        return 1;
    };
'
expect_err 'TypeError' '
    func main() Str {};
'
expect_err 'SyntaxError' '
    func main();
'


describe 'Top Level Statements with main Function'

expect_err 'SyntaxError' '
    func f () {};
    func main() {
        print("hi");
    };
    f();
'
expect_err 'SyntaxError' '
    func f () {};
    f();
    func main() {
        print("hi");
    };
'
expect_err 'SyntaxError' '
    func main() {
        print("hi");
    };
    if true {};
'
expect_err 'SyntaxError' '
    if false {};
    func main() {
        print("hi");
    };
'
expect 'Hello' '
    const s = "Hello";
    func main() {
        print(s);
    };
'
expect '16' '
    const s = 16;
    func main() {
        print(s.Str());
    };
'