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
        print_int(3);
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
          print_int(a + b + c);
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
    var i = 0;
    fn f() {
        for {
            i = i + 1;
            if i > 2 { return };
            print_int(i);
        };
    };
    f();
'
expect '1' '
    fn f(): Int {
        return 1;
    };
    print_int(f());
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
    fn f(): Int {
      print_int(1);
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
    print_int(f());
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
    print_bool(f());
'
expect 'true' '
    fn f(): Bool {
      return true;
    };
    print_bool(f());
'
expect_err 'TypeError' '
    fn f(): Str {
        return "";
    };
    print_bool(f());
'
expect_err 'TypeError' '
    fn f(): Void {};
    print_bool(f());
'
expect '16' '
    fn square(n: Int): Int {
        return n * n;
    };
    print_int(square(4));
'
expect '17' '
    fn square(n: Int): Int {
        return n * n;
    };
    print_int(square(4) + square(-1));
'
expect '90' '
    fn sum(a: Int, b: Int, c: Int): Int {
        return a + b + c;
    };
    print_int(sum(1, 2, 3) * sum(4, 5, 6));
'
expect '49' '
    fn f(n: Int): Int {
        return n;
    };
    print_int(f(4));
    print_int(f(4) + f(5));
'
expect '49' '
    fn g() {
        print_int(49);
        return;
        print_bool(true);
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

_="
describe 'Default Arguments'

expect '123' '
    fn f(a: Int, b: Int = 2, c: Int = 3) {
        print_int(a);
        print_int(b);
        print_int(c);
    };
    f(1);
'
expect '14' '
    const a = 1;
    fn f(a: Int = a) {
        print_int(a);
    };
    f();
    f(4);
'
expect '44' '
    const u = 1;
    fn f(a: Int, b: Int = a-u) {
        print_int(a);
        print_int(b);
    };
    f(4);
'
expect '463' '
    fn f(a: Int, b=a+2, c=3): Int {
        print_int(a);
        print_int(b);
        print_int(c);
        return c;
    };
    f(4);
'
"