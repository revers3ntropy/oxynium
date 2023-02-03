describe 'main Function'

expect 'hi' '
    def main() {
        print("hi");
    }
'
expect_err 'TypeError' '
    def main(a: Int) {}
'
expect_err 'TypeError' '
    def main() {
        return "hi"
    }
'
expect_err 'TypeError' '
    def main() {
        return 1;
    };
'
expect_err 'TypeError' '
    def main() Str {};
'
expect_err 'SyntaxError' '
    def main();
'


describe 'Top Level Statements with main Function'

expect_err 'SyntaxError' '
    def f () {};
    def main() {
        print("hi");
    };
    f();
'
expect_err 'SyntaxError' '
    def f () {};
    f();
    def main() {
        print("hi");
    };
'
expect_err 'SyntaxError' '
    def main() {
        print("hi");
    };
    if true {};
'
expect_err 'SyntaxError' '
    if false {};
    def main() {
        print("hi");
    };
'
expect 'Hello' '
    const s = "Hello";
    def main() {
        print(s);
    };
'
expect '16' '
    const s = 16;
    def main() {
        print(s.Str());
    };
'