describe 'Function Declarations'

expect 'fn a()' ''
expect 'fn a(a: Int, b: Bool, c: Str)' ''
expect 'fn a(): Void' ''
expect 'fn a(a: Int): Str' ''
expect_err 'fn a(a): Str' 'SyntaxError'
expect_err 'fn a(a)' 'SyntaxError'
expect_err 'fn()' 'SyntaxError'
expect_err 'fn 0()' 'SyntaxError'
expect_err 'fn 0g()' 'SyntaxError'
expect_err 'fn g(); fn g();' 'TypeError'


describe 'Writing Functions'

expect '
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
' '12345'

expect '
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
' '3'

expect '
    fn log(msg: Str) {
        print(msg);
    };
    log("Hello");
' 'Hello'

expect '
      fn sum_and_log(a: Int, b: Int, c: Int) {
          print_int(a + b + c);
      };
      sum_and_log(5, 8, 9);
' '22'

expect '
    fn log(msg1: Str, msg2: Str, msg3: Str) {
        print(msg1);
        print(msg2);
        print(msg3);
    };
    log("Hello", " World", "!");
' 'Hello World!'

expect_err '
    fn f(a: Int) {
        a = 2;
    };
f(1);
' 'TypeError'


describe 'Return'

expect '
    fn f() {
        return;
        print("hi");
    };
    f();
' ''

expect '
    fn f() {
        print("1");
        return;
        print("2");
    };
    f();
' '1'

expect '
    var i = 0;
    fn f() {
        for {
            i = i + 1;
            if i > 2 { return };
            print_int(i);
        };
    };
    f();
' '12'
expect '
    fn f(): Int {
        return 1;
    };
    print_int(f());
' '1'

expect_err '
    fn f(): Int {
      return "";
    };
' 'TypeError'
expect_err '
    fn f(): Int {
        return "";
    };
' 'TypeError'
expect_err '
    fn f() {
      return "";
    };
' 'TypeError'
expect_err '
    fn f(): Int {
        print("hi");
        return;
    };
' 'TypeError'
expect_err '
    fn f(): Int {
      print_int(1);
    };
' 'TypeError'
expect_err '
    fn f(): Void {
      return 1;
    };
' 'TypeError'
expect '
    fn f(): Void {
      return;
    };
    f();
' ''
expect '
    fn f(): Void {
      1;
    };
    f();
' ''
expect '
    fn f(): Void {
      print("hi");
    };
    f();
' 'hi'
expect '
    fn f(): Int {
      return 1;
      print("hi");
    };
    print_int(f());
' '1'
expect '
    fn f(): Str {
      return "hi";
    };
    print(f());
' 'hi'
expect '
    fn f(): Bool {
      return 1 == 2;
    };
    print_bool(f());
' 'false'
expect '
    fn f(): Bool {
      return true;
    };
    print_bool(f());
' 'true'
expect_err '
    fn f(): Str {
        return "";
    };
    print_bool(f());
' 'TypeError'
expect_err '
    fn f(): Void {};
    print_bool(f());
' 'TypeError'
expect '
    fn square(n: Int): Int {
        return n * n;
    };
    print_int(square(4));
' '16'
expect '
    fn square(n: Int): Int {
        return n * n;
    };
    print_int(square(4) + square(-1));
' '17'
expect '
    fn sum(a: Int, b: Int, c: Int): Int {
        return a + b + c;
    };
    print_int(sum(1, 2, 3) * sum(4, 5, 6));
' '90'
expect '
    fn f(n: Int): Int {
        return n;
    };
    print_int(f(4));
    print_int(f(4) + f(5));
' '49'

expect '
    fn g() {};
    fn f(n: Int): Void {
        return g();
    };
    f(4);
' '49'

expect '
    fn g(a: Str) {};
    fn f(n: Int, m: Int): Void {
      return g("");
    };
    f(4, 6);
' ''