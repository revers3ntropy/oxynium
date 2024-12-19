describe 'Return from Functions'

expect '12' '
    def f() {
        let mut i = 0;
        while {
            i = i + 1;
            if i > 2 { return };
            print(i.Str());
        };
    };
    f();
'
expect '1' '
    def f() Int {
        return 1;
    };
    print(f().Str());
'

expect_err 'TypeError' '
    def f() Int {
      return "";
    };
'
expect_err 'TypeError' '
    def f() Int -> ""
'
expect_err 'TypeError' '
    def f() -> 1
    print(f())
'
expect_err 'h' '
    def f() -> "h"
    print(f())
'
expect_err 'TypeError' '
    def f() Int {
        return "";
    };
'
expect_err 'TypeError' '
    def f() {
      return "";
    };
'
expect_err 'TypeError' '
    def f() Int {
        print("hi");
        return;
    };
'
expect_err 'TypeError' '
    def f() Str {
        print(1.Str());
    };
'
expect_err 'TypeError' '
    def f() Void {
        return 1;
    };
'
expect '' '
    def f() Void {
        return;
    };
    f();
'
expect '' '
    def f() Void {
        1;
    };
    f();
'
expect 'hi' '
    def f() Void {
        print("hi");
    };
    f();
'
expect 'hi' '
    def f() Str {
      return "hi";
    };
    print(f());
'
expect 'false' '
    def f() Bool {
      return 1 == 2
    }
    print(f().Str())
'
expect 'true' '
    def f() -> true
    print(f().Str())
'
expect '' '
    def f() Str -> ""
    print(f().Str())
'
expect_err 'TypeError' '
    def f() Str {
        return "";
    };
    print((f() + 2).Str());
'
expect_err 'TypeError' '
    def f() Void {};
    print(f().Str());
'
expect '16' '
    def square(n: Int) Int {
        return n * n;
    };
    print(square(4).Str());
'
expect '17' '
    def square(n: Int) Int {
        return n * n;
    };
    print((square(4) + square(-1)).Str());
'
expect '90' '
    def sum(a: Int, b: Int, c: Int) Int {
        return a + b + c;
    };
    print((sum(1, 2, 3) * sum(4, 5, 6)).Str());
'
expect '49' '
    def f(n: Int) Int {
        return n;
    };
    print(f(4).Str());
    print((f(4) + f(5)).Str());
'
expect '' '
    def g(a: Str) {};
    def f(n: Int, m: Int) Void {
      return g("");
    };
    f(4, 6);
'


describe 'Do Not Allow Return in Top-Level'

expect_err 'SyntaxError' '
    return;
'
expect_err 'SyntaxError' '
    return 1;
'
expect_err 'SyntaxError' '
    return 1 + 2;
'
expect_err 'SyntaxError' '
    def f () -> 1
    return
'
expect_err 'SyntaxError' '
    def main() {
        return
    }
    return
'
expect_err 'SyntaxError' '
    def main() {
        return
    }
    return 1
'


describe 'All Execution Paths Must Return'

expect_err 'TypeError' '
    def f() Int {
        if true {
            return 1
        } else {

        }
    }
'
expect '' '
    def f() Int {
        if true {
            return 1
        } else {
            return 2
        }
    }
'
expect_err 'TypeError' '
    def f() Int {}
'
expect '' '
    def f() Int {
        return 1
    }
'
expect_err 'TypeError' '
    def f() Int {
        if true {
            return 1
        }
    }
'
expect_err 'TypeError' '
    def f() Int {
        if true {
            if false {
                return 1
            }
        } else {
            return 2
        }
    }
'
expect '' '
    def f() Int {
        if true {
            if false {
                return 1
            }
        }
        return 2
    }
'
expect '' '
    def f() Int {
        while {
            return 1
        }
    }
'
expect '' '
    def f(a: Bool) Int {
        while a {
            return 1
        }
        return 2
    }
'
expect_err 'TypeError' '
    def f(a: Bool) Int {
        while a {
            return 1
        }
    }
'


describe 'Unreachable code throws error'

expect_err 'SyntaxError' '
    def f() {
        return
        print("hi")
    }
'
expect_err 'SyntaxError' '
    def f() Int {
        return 1
        print("hi")
    }
'
expect_err 'SyntaxError' '
    def f() Int {
        return 1
        return 2
    }
'