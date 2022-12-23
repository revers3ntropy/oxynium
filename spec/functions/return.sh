describe 'Return from Functions'

expect '' '
    fn f() {
        return;
        print("hi");
    };
    f();
'

expect '1' '
    fn f() {
        print("1");
        return;
        print("2");
    };
    f();
'

expect '12' '
    fn f() {
        let mut i = 0;
        while {
            i = i + 1;
            if i > 2 { return };
            print(i.str());
        };
    };
    f();
'
expect '1' '
    fn f() Int {
        return 1;
    };
    print(f().str());
'

expect_err 'TypeError' '
    fn f() Int {
      return "";
    };
'
expect_err 'TypeError' '
    fn f() Int {
        return "";
    };
'
expect_err 'TypeError' '
    fn f() {
      return "";
    };
'
expect_err 'TypeError' '
    fn f() Int {
        print("hi");
        return;
    };
'
expect_err 'TypeError' '
    fn f() Str {
        print(1.str());
    };
'
expect_err 'TypeError' '
    fn f() Void {
        return 1;
    };
'
expect '' '
    fn f() Void {
        return;
    };
    f();
'
expect '' '
    fn f() Void {
        1;
    };
    f();
'
expect 'hi' '
    fn f() Void {
        print("hi");
    };
    f();
'
expect '1' '
    fn f() Int {
        return 1;
        print("hi");
    };
    print(f().str());
'
expect_err 'TypeError' '
    fn f() Int {
        return 1;
        return true;
    };
'
expect 'hi' '
    fn f() Str {
      return "hi";
    };
    print(f());
'
expect 'false' '
    fn f() Bool {
      return 1 == 2;
    };
    print(f().str());
'
expect 'true' '
    fn f() Bool {
      return true;
    };
    print(f().str());
'
expect '' '
    fn f() Str {
        return "";
    };
    print(f().str());
'
expect_err 'TypeError' '
    fn f() Str {
        return "";
    };
    print((f() + 2).str());
'
expect_err 'TypeError' '
    fn f() Void {};
    print(f().str());
'
expect '16' '
    fn square(n: Int) Int {
        return n * n;
    };
    print(square(4).str());
'
expect '17' '
    fn square(n: Int) Int {
        return n * n;
    };
    print((square(4) + square(-1)).str());
'
expect '90' '
    fn sum(a: Int, b: Int, c: Int) Int {
        return a + b + c;
    };
    print((sum(1, 2, 3) * sum(4, 5, 6)).str());
'
expect '49' '
    fn f(n: Int) Int {
        return n;
    };
    print(f(4).str());
    print((f(4) + f(5)).str());
'
expect '49' '
    fn g() {
        print(49.str());
        return;
        print(true.str());
    };
    fn f(n: Int) Void {
        return g();
    };
    f(4);
'
expect '' '
    fn g(a: Str) {};
    fn f(n: Int, m: Int) Void {
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
    return;
    fn main() {}
'
expect_err 'SyntaxError' '
    fn main() {
        return
    }
    return
'
expect_err 'SyntaxError' '
    fn main() {
        return
    }
    return 1
'


describe 'All Execution Paths Must Return'

expect_err 'TypeError' '
    fn f() Int {
        if true {
            return 1
        } else {

        }
    }
'
expect '' '
    fn f() Int {
        if true {
            return 1
        } else {
            return 2
        }
    }
'
expect_err 'TypeError' '
    fn f() Int {}
'
expect '' '
    fn f() Int {
        return 1
    }
'
expect_err 'TypeError' '
    fn f() Int {
        if true {
            return 1
        }
    }
'
expect_err 'TypeError' '
    fn f() Int {
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
    fn f() Int {
        if true {
            if false {
                return 1
            }
        }
        return 2
    }
'
expect '' '
    fn f() Int {
        while {
            return 1
        }
    }
'
expect '' '
    fn f(a: Bool) Int {
        while a {
            return 1
        }
        return 2
    }
'
expect_err 'TypeError' '
    fn f(a: Bool) Int {
        while a {
            return 1
        }
    }
'