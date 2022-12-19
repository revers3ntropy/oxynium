describe 'Function Declarations'

expect '' 'fn a()'
expect '' 'fn a(a: Int, b: Bool, c: Str)'
expect '' 'fn a(): Void'
expect '' 'fn a(a: Int): Str'
expect_err 'SyntaxError' 'fn a(a): Str'
expect_err 'SyntaxError' 'fn a(a)'
expect_err 'SyntaxError' 'fn()'
expect_err 'SyntaxError' 'fn 0()'
expect_err 'SyntaxError' 'fn 0g()'
expect_err 'TypeError' 'fn g(); fn g();'


describe 'Writing Functions'

expect '12345' '
    fn g() {
        print("3");
    };

    fn f() {
        print("2");
        g();
        print("4");
    };

    print("1");
    f();
    print("5");
'
expect '3' '
    fn g() {
        2; // just push a value to the stack
        // Ensures the stack is cleared when the function returns
    };
    fn f() {
        1;
        g();
        print(3.str());
        3;
    };
    f();
'
expect 'Hello' '
    fn log(msg: Str) {
        print(msg);
    };
    log("Hello");
'
expect '22' '
      fn sum_and_log(a: Int, b: Int, c: Int) {
          print((a + b + c).str());
      };
      sum_and_log(5, 8, 9);
'
expect 'Hello World!' '
    fn log(msg1: Str, msg2: Str, msg3: Str) {
        print(msg1);
        print(msg2);
        print(msg3);
    };
    log("Hello", " World", "!");
'
expect_err 'TypeError' '
    fn f(a: Int) {
        a = 2;
    };
f(1);
'


describe 'Return'

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
    fn f(): Int {
        return 1;
    };
    print(f().str());
'

expect_err 'TypeError' '
    fn f(): Int {
      return "";
    };
'
expect_err 'TypeError' '
    fn f(): Int {
        return "";
    };
'
expect_err 'TypeError' '
    fn f() {
      return "";
    };
'
expect_err 'TypeError' '
    fn f(): Int {
        print("hi");
        return;
    };
'
expect_err 'TypeError' '
    fn f(): Str {
        print(1.str());
    };
'
expect_err 'TypeError' '
    fn f(): Void {
        return 1;
    };
'
expect '' '
    fn f(): Void {
        return;
    };
    f();
'
expect '' '
    fn f(): Void {
        1;
    };
    f();
'
expect 'hi' '
    fn f(): Void {
        print("hi");
    };
    f();
'
expect '1' '
    fn f(): Int {
        return 1;
        print("hi");
    };
    print(f().str());
'
expect_err 'TypeError' '
    fn f(): Int {
        return 1;
        return true;
    };
'
expect 'hi' '
    fn f(): Str {
      return "hi";
    };
    print(f());
'
expect 'false' '
    fn f(): Bool {
      return 1 == 2;
    };
    print(f().str());
'
expect 'true' '
    fn f(): Bool {
      return true;
    };
    print(f().str());
'
expect '' '
    fn f(): Str {
        return "";
    };
    print(f().str());
'
expect_err 'TypeError' '
    fn f(): Str {
        return "";
    };
    print((f() + 2).str());
'
expect_err 'TypeError' '
    fn f(): Void {};
    print(f().str());
'
expect '16' '
    fn square(n: Int): Int {
        return n * n;
    };
    print(square(4).str());
'
expect '17' '
    fn square(n: Int): Int {
        return n * n;
    };
    print((square(4) + square(-1)).str());
'
expect '90' '
    fn sum(a: Int, b: Int, c: Int): Int {
        return a + b + c;
    };
    print((sum(1, 2, 3) * sum(4, 5, 6)).str());
'
expect '49' '
    fn f(n: Int): Int {
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
    fn f(n: Int): Void {
        return g();
    };
    f(4);
'
expect '' '
    fn g(a: Str) {};
    fn f(n: Int, m: Int): Void {
      return g("");
    };
    f(4, 6);
'
expect_err 'SyntaxError' '
    fn g() {
        fn f() {};
    };
'
expect_err 'SyntaxError' '
    fn g() {
        fn f() {
            fn h() {};
        };
    };
'


describe 'Default Arguments'

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
    fn f(a: Bool, b: Int = 2, c: Str = "hi"): Int {
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
expect_err 'SyntaxError' '
    extern fn main();
'