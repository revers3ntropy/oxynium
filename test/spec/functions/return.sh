describe 'Return from Functions'

expect '' '
    func f() {
        return;
        print("hi");
    };
    f();
'

expect '1' '
    func f() {
        print("1");
        return;
        print("2");
    };
    f();
'

expect '12' '
    func f() {
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
    func f() Int {
        return 1;
    };
    print(f().Str());
'

expect_err 'TypeError' '
    func f() Int {
      return "";
    };
'
expect_err 'TypeError' '
    func f() Int {
        return "";
    };
'
expect_err 'TypeError' '
    func f() {
      return "";
    };
'
expect_err 'TypeError' '
    func f() Int {
        print("hi");
        return;
    };
'
expect_err 'TypeError' '
    func f() Str {
        print(1.Str());
    };
'
expect_err 'TypeError' '
    func f() Void {
        return 1;
    };
'
expect '' '
    func f() Void {
        return;
    };
    f();
'
expect '' '
    func f() Void {
        1;
    };
    f();
'
expect 'hi' '
    func f() Void {
        print("hi");
    };
    f();
'
expect '1' '
    func f() Int {
        return 1;
        print("hi");
    };
    print(f().Str());
'
expect_err 'TypeError' '
    func f() Int {
        return 1;
        return true;
    };
'
expect 'hi' '
    func f() Str {
      return "hi";
    };
    print(f());
'
expect 'false' '
    func f() Bool {
      return 1 == 2;
    };
    print(f().Str());
'
expect 'true' '
    func f() Bool {
      return true;
    };
    print(f().Str());
'
expect '' '
    func f() Str {
        return "";
    };
    print(f().Str());
'
expect_err 'TypeError' '
    func f() Str {
        return "";
    };
    print((f() + 2).Str());
'
expect_err 'TypeError' '
    func f() Void {};
    print(f().Str());
'
expect '16' '
    func square(n: Int) Int {
        return n * n;
    };
    print(square(4).Str());
'
expect '17' '
    func square(n: Int) Int {
        return n * n;
    };
    print((square(4) + square(-1)).Str());
'
expect '90' '
    func sum(a: Int, b: Int, c: Int) Int {
        return a + b + c;
    };
    print((sum(1, 2, 3) * sum(4, 5, 6)).Str());
'
expect '49' '
    func f(n: Int) Int {
        return n;
    };
    print(f(4).Str());
    print((f(4) + f(5)).Str());
'
expect '49' '
    func g() {
        print(49.Str());
        return;
        print(true.Str());
    };
    func f(n: Int) Void {
        return g();
    };
    f(4);
'
expect '' '
    func g(a: Str) {};
    func f(n: Int, m: Int) Void {
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
    func main() {}
'
expect_err 'SyntaxError' '
    func main() {
        return
    }
    return
'
expect_err 'SyntaxError' '
    func main() {
        return
    }
    return 1
'


describe 'All Execution Paths Must Return'

expect_err 'TypeError' '
    func f() Int {
        if true {
            return 1
        } else {

        }
    }
'
expect '' '
    func f() Int {
        if true {
            return 1
        } else {
            return 2
        }
    }
'
expect_err 'TypeError' '
    func f() Int {}
'
expect '' '
    func f() Int {
        return 1
    }
'
expect_err 'TypeError' '
    func f() Int {
        if true {
            return 1
        }
    }
'
expect_err 'TypeError' '
    func f() Int {
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
    func f() Int {
        if true {
            if false {
                return 1
            }
        }
        return 2
    }
'
expect '' '
    func f() Int {
        while {
            return 1
        }
    }
'
expect '' '
    func f(a: Bool) Int {
        while a {
            return 1
        }
        return 2
    }
'
expect_err 'TypeError' '
    func f(a: Bool) Int {
        while a {
            return 1
        }
    }
'